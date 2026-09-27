# Base on https://github.com/mcavus/Kernighan-Lin.git
from enum import Enum


class Partition(Enum):
    A = 1,
    B = 2,


class Edge:
    def __init__(self, left_id: int, right_id: int):
        self.left_id: int = left_id
        self.right_id: int = right_id
        self.left_v: Vertex = None
        self.right_v: Vertex = None

    def is_cut(self) -> bool:
        """
            判断当前 edge 是否被划分 cut
        """
        return self.left_v.partition != self.right_v.partition
    

class Vertex:
    def __init__(self, id: int):
        self.id: int = id
        self.edges: list[Edge] = []
        self.partition: Partition = None

    def get_D_value(self) -> int:
        """
            计算 Vertex 的 D 值

            对于任意顶点 v
            - 外部代价 Ev：指所有连接到另一侧的边数。
            - 外部代价 Iv：指所有连接到同一侧的边数。

            有：
            $$
               D(v) = E_v - I_v
            $$

            若 D 很大，说明这个点跟另一侧的连接多，跟自己这侧连接少 —— 把它换到另一侧会更可能降低 cut
        """
        D_value = 0
        for edge in self.edges:
            if edge.is_cut():
                D_value += 1
            else:
                D_value -= 1
        return D_value

    def shared_edge_size(self, other) -> int:
        for edge in self.edges:
            if self.id == edge.left_id and other.id == edge.right_id:
                return 1
            if self.id == edge.right_id and other.id == edge.left_id:
                return 1
        return 0

    def add_edge(self, edge: Edge):
        for present_edge in self.edges:
            # 判断是否重重添加
            if present_edge.left_id == edge.right_id and present_edge.right_id == edge.left_id:
                return
        
        self.edges.append(edge)


class Graph:
    def __init__(self, vertices: list[Vertex], edges: list[Edge]):
        self.vertices = vertices
        self.edges = edges

        vertex_dict = {v.id: v for v in self.vertices}
        
        for edge in self.edges:
            # left_v <===egde===> right_v
            left_v = vertex_dict[edge.left_id]
            edge.left_v = left_v
            left_v.add_edge(edge)

            right_v = vertex_dict[edge.right_id]
            edge.right_v = right_v
            right_v.add_edge(edge)

    @staticmethod
    def load_data(filename: str):
        file = open(filename, 'r')
        edges = []
        vertices = []
        seen_vertex_ids = []
        
        # 文件的每一行
        for line in list(file):
            # 解析为两个整数，作为 left id 和 right id
            v_list = line.split()
            left_id = int(v_list[0])
            right_id = int(v_list[1])
            
            # 二者用 edge 连接
            edges.append(Edge(left_id, right_id))
            
            # 创建新的结点
            if left_id not in seen_vertex_ids:
                vertices.append(Vertex(left_id))
                seen_vertex_ids.append(left_id)
                
            if right_id not in seen_vertex_ids:
                vertices.append(Vertex(right_id))
                seen_vertex_ids.append(right_id)
        
        return Graph(vertices, edges)

    def get_partition_cost(self):
        """
            获取当前划分情况的 cost，即统计所有 is_cut edge 的数量 
        """
        return sum(map(Edge.is_cut, self.edges))
    
    def vertex_size(self) -> int:
        return len(self.vertices)
    

class KernighanLin():
    def __init__(self, graph: Graph):   
        self.graph = graph

    def partition(self):
        vertex_size = self.graph.vertex_size()

        # 初始化划分，A B 两组各一半
        for i in range(vertex_size // 2):
            self.graph.vertices[i].partition = Partition.A
        for i in range(vertex_size // 2, vertex_size):
            self.graph.vertices[i].partition = Partition.B

        print(f"Initial partition cost: {self.graph.get_partition_cost()}")

        p = 0
        total_gain = 0

        while True:
            # 每次迭代讲 A B 两种 vertex 分开保存
            group_a: list[Vertex] = []
            group_b: list[Vertex] = []

            for vertex in self.graph.vertices:
                if vertex.partition == Partition.A:
                    group_a.append(vertex)
                elif vertex.partition == Partition.B:
                    group_b.append(vertex)
                else:
                    assert False
            
            # 计算当前所有结点的 D 值
            D_values = { v.id : v.get_D_value() for v in self.graph.vertices }
            gains = [] # [ (a, b, gain) ]

            for _ in range(vertex_size // 2):
                # 每次循环找到一对交换增益最大的结点
                max_gain = -1 * float("inf")
                pair = None

                for a in group_a:
                    for b in group_b:
                        # 计算两个组中任意两个结点的 gain
                        c_ab = a.shared_edge_size(b)
                        gain = D_values[a.id] + D_values[b.id] - (2 * c_ab)

                        if gain > max_gain:
                            max_gain = gain
                            pair = (a, b)
                
                # 找到最大的 gain 组合
                (a, b) = pair
                group_a.remove(a)
                group_b.remove(b)
                gains.append((a, b, max_gain))

                # 更新 D_values
                for x in group_a:
                    # 对集合 A，需要移除 a，加入 b
                    c_xa = x.shared_edge_size(a)
                    c_xb = x.shared_edge_size(b)
                    D_values[x.id] += 2 * (c_xa) - 2 * (c_xb)

                for y in group_b:
                    # 对集合 A，需要移除 a，加入 b
                    c_ya = y.shared_edge_size(a)
                    c_yb = y.shared_edge_size(b)
                    D_values[y.id] += 2 * (c_yb) - 2 * (c_ya)

            # find j that maximizes the sum g_max
            # gains 数组保存交换结点以及其增益
            # 要找到交换前多个组，可以获得最大增益
            g_max = -1 * float("inf")
            jmax = 0
            for j in range(1, len(gains) + 1):
                g_sum = sum(map(lambda g: g[2], gains[:j]))
                if g_sum > g_max:
                    g_max = g_sum
                    jmax = j
            
            if g_max > 0:
                for a, b, _ in gains[:jmax]:
                    a.partition = Partition.B
                    b.partition = Partition.A
                p += 1
                total_gain += g_max

                print(f"Pass: {p} \t\t\tGain: {g_max}")
            else:
                break

        print(f"Total passes: {p} \t\tTotal gain: {total_gain} + Final partition cost: {self.graph.get_partition_cost()}")

    
if __name__ == "__main__":
    graph = Graph.load_data("./data/graph.txt")
    kl = KernighanLin(graph)
    kl.partition()