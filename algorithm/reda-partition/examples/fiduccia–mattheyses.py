from enum import Enum
from collections import defaultdict, deque


class Partition(Enum):
    A = 1
    B = 2


class Edge:
    def __init__(self, left_id: int, right_id: int):
        self.left_id: int = left_id
        self.right_id: int = right_id
        self.left_v: Vertex = None
        self.right_v: Vertex = None

    def is_cut(self) -> bool:
        return self.left_v.partition != self.right_v.partition


class Vertex:
    def __init__(self, id: int):
        self.id: int = id
        self.edges: list[Edge] = []
        self.partition: Partition = None

        # FM-specific state
        self.locked: bool = False
        self.current_gain: int = 0
        self.bucket_gain: int | None = None  # the gain value where this vertex currently sits

    def __repr__(self):
        return f"V({self.id}, p={self.partition}, g={self.current_gain}, locked={self.locked})"

    def add_edge(self, edge: Edge):
        # avoid duplicate same-edge reversed
        for present_edge in self.edges:
            if present_edge.left_id == edge.right_id and present_edge.right_id == edge.left_id:
                return
        self.edges.append(edge)

    def get_D_value(self) -> int:
        # D(v) = E_v - I_v
        D_value = 0
        for edge in self.edges:
            if edge.is_cut():
                D_value += 1
            else:
                D_value -= 1
        return D_value

    def neighbors(self):
        for e in self.edges:
            yield e.left_v if e.right_v is self else e.right_v

    def shared_edge_size(self, other) -> int:
        return len(set(self.edges).intersection(other.edges))


class Graph:
    def __init__(self, vertices: list[Vertex], edges: list[Edge]):
        self.vertices = vertices
        self.edges = edges

        vertex_dict = {v.id: v for v in self.vertices}

        for edge in self.edges:
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

        for line in list(file):
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            v_list = line.split()
            left_id = int(v_list[0])
            right_id = int(v_list[1])

            edges.append(Edge(left_id, right_id))

            if left_id not in seen_vertex_ids:
                vertices.append(Vertex(left_id))
                seen_vertex_ids.append(left_id)

            if right_id not in seen_vertex_ids:
                vertices.append(Vertex(right_id))
                seen_vertex_ids.append(right_id)

        return Graph(vertices, edges)

    def get_partition_cost(self):
        return sum(map(Edge.is_cut, self.edges))

    def vertex_size(self) -> int:
        return len(self.vertices)


class FiducciaMattheyses:
    def __init__(self, graph: Graph, balance_tolerance: int = 0):
        """
        balance_tolerance: allowed difference in sizes between partitions (|sizeA - sizeB| <= balance_tolerance)
        For exact-equal split use balance_tolerance=0 (this enforces exact equality for chosen prefix).
        """
        self.graph = graph
        self.balance_tolerance = balance_tolerance

    def _init_partition_equal(self):
        """
            设置结点 bucket_gain 为 None，current_gain = 0，locked = False
            并且根据结点的顺序平均分为 A / B 两组
        """
        n = self.graph.vertex_size()
        for i, v in enumerate(self.graph.vertices):
            v.locked = False
            v.bucket_gain = None
            v.current_gain = 0
            # initial equal split
            v.partition = Partition.A if i < n // 2 else Partition.B

    def _compute_initial_gains(self):
        """
            结点 依次计算 get_D_value 方法，赋值给 current_gain
        """
        for v in self.graph.vertices:
            v.current_gain = v.get_D_value()

    def _make_buckets(self):
        """
            将所有相同 gain 的结点放在一个集合中
            并且找到最大的 gain
        """
        buckets = defaultdict(set)
        max_gain = -10**9
        for v in self.graph.vertices:
            g = v.current_gain
            v.bucket_gain = g
            buckets[g].add(v)
            if g > max_gain:
                max_gain = g
        return buckets, max_gain

    def _remove_from_bucket(self, buckets, v: Vertex):
        """
            将 v 从 buckets 移出，并且设置 v.bucket_gain = None
        """
        g = v.bucket_gain
        if g is None:
            return
        s = buckets.get(g)
        if s is None:
            return
        if v in s:
            s.remove(v)
        v.bucket_gain = None

    def _add_to_bucket(self, buckets, v: Vertex, g: int):
        """
            g 插入 buckets，并且设置 v.bucket_gain = g， v.current_gain = g
        """
        buckets[g].add(v)
        v.bucket_gain = g
        v.current_gain = g

    def _find_max_gain(self, buckets, current_max):
        """
            找最大的 “有效” gain
            输入 current_max 是目前最大的，但不一定有效，需要检查：

            - buckets 是否含有 current_max 所对应的结点集合
            - buckets 有这个集合，但集合不可以是空的

            如果 current_max 无效，检查整个 buckets 最大并且集合不为空的 gain
            如果没有将返回 None 
        """
        # if current_max bucket empty, scan downward until find non-empty or full range exhausted
        # We will scan from current_max down to min gain in buckets keys
        if not buckets:
            return None
        keys = buckets.keys()
        if not keys:
            return None
        max_k = current_max
        # if current_max not in keys or empty, adjust
        if max_k not in buckets or len(buckets[max_k]) == 0:
            # scan keys for available max
            available = [k for k, s in buckets.items() if len(s) > 0]
            if not available:
                return None
            max_k = max(available)
        return max_k

    def partition(self):
        n = self.graph.vertex_size()
        if n == 0:
            return

        self._init_partition_equal()
        print(f"[FM] Initial partition cost: {self.graph.get_partition_cost()}")

        passes = 0
        total_gain = 0

        while True:
            # 每次迭代开始，un locked 所有结点，并且重新计算 current_gain
            passes += 1
            for v in self.graph.vertices:
                v.locked = False
            self._compute_initial_gains()

            # 初始化 buckets
            buckets, max_gain = self._make_buckets()

            move_sequence = []  # list of (vertex, from_part, to_part, gain)
            cumulative_gains = []

            current_sizes = {
                Partition.A: sum(1 for v in self.graph.vertices if v.partition == Partition.A),
                Partition.B: sum(1 for v in self.graph.vertices if v.partition == Partition.B),
            }

            # total moves in a pass <= n
            for move_idx in range(n):
                # 取出最大的“有效” gain
                mg = self._find_max_gain(buckets, max_gain)
                if mg is None:
                    break
                
                # 取出这个 buckets 中对应的一个结点
                v = next(iter(buckets[mg]))
                # 将其移除
                self._remove_from_bucket(buckets, v)
                v.locked = True

                # 将这个结点移动
                from_part = v.partition
                to_part = Partition.A if from_part == Partition.B else Partition.B
                move_gain = v.current_gain

                # 更新 A, B 集合的数量
                v.partition = to_part
                current_sizes[from_part] -= 1
                current_sizes[to_part] += 1

                # 记录移动
                move_sequence.append((v, from_part, to_part, move_gain))
                cumulative_gains.append(move_gain + (cumulative_gains[-1] if cumulative_gains else 0))

                # 结点 v 更新后，其所有邻居的 D 值也会改变
                for nbr in v.neighbors():
                    # 被移动过的点不会被选中，无需更新 D 
                    if nbr.locked:
                        continue
                    
                    # 重新计算 gain，重新插入 buckets
                    old_g = nbr.current_gain
                    self._remove_from_bucket(buckets, nbr)
                    new_g = nbr.get_D_value()
                    self._add_to_bucket(buckets, nbr, new_g)

                # 观察每次移动结点是否有更大的 gain 出现
                if mg > max_gain:
                    max_gain = mg

            # 这次迭代的可移动方案已经确定，开始找到最佳的组合
            best_gain = -10**9
            best_k = 0

            # 撤回之前所有的移动（他们移动只是为了计算 D，但现在要回到原来的状态）
            for (v, frm, to, g) in reversed(move_sequence):
                v.partition = frm

            # Now simulate prefixes and enforce balance
            sizeA = sum(1 for v in self.graph.vertices if v.partition == Partition.A)
            sizeB = sum(1 for v in self.graph.vertices if v.partition == Partition.B)

            # move_sequence 表示的是一组可以移动的方案，但由于后面的移动是在之前的基础得到的，所以选择方案必须从 0 开始到 k 个截止
            # 每次循环观察前 k 次移动的效果
            for k in range(1, len(move_sequence) + 1):
                # 因为上一次迭代已经完成了 k - 1 次移动，这次只需要完成第 k 次
                v, frm, to, g = move_sequence[k - 1]
                v.partition = to
                if frm == Partition.A:
                    sizeA -= 1
                else:
                    sizeB -= 1
                if to == Partition.A:
                    sizeA += 1
                else:
                    sizeB += 1

                # 计算 A B 组合的平衡约束，如果不满足的话，就不可以保存前 k 次的移动作为最后的更新方案
                if abs(sizeA - sizeB) <= self.balance_tolerance:
                    # 如果满足，判断前 k 次移动的组合是否为当前最佳方案
                    if cumulative_gains[k - 1] > best_gain:
                        best_gain = cumulative_gains[k - 1]
                        best_k = k

            # 如果 best_gain > 0，说明这次更新是有作用的，否则说明无效，那么就撤回所有的更新
            if best_gain > 0:
                total_gain += best_gain
                # for k in range(1, len(move_sequence) + 1): 上面这个循环为了测试所有组合的效果，实际上应用了所有的方案
                # 而实际最后采用的是前 best_k 个，所以将 best_k 到最后所有的移动方案撤回
                for idx in range(best_k, len(move_sequence)):
                    v, frm, to, g = move_sequence[idx]
                    v.partition = frm
                print(f"[FM] Pass {passes}: applied {best_k} moves, gain={best_gain}")
            else:
                # 所有方案都无效，撤回所有的
                for (v, frm, to, g) in move_sequence:
                    v.partition = frm
                print(f"[FM] Pass {passes}: no improvement (best_gain={best_gain}). Stopping.")
                passes -= 1 
                break

        print(f"[FM] Total passes: {passes}, Total gain: {total_gain}, Final cut cost: {self.graph.get_partition_cost()}")


if __name__ == "__main__":
    graph = Graph.load_data("./data/graph.txt")
    kl = FiducciaMattheyses(graph)
    kl.partition()