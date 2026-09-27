use std::{collections::HashMap, fmt::Debug, fs, path::Path, str::FromStr};

use crate::Error;

#[derive(Debug, Clone)]
pub struct Graph<I> {
    pub vertices: HashMap<I, Vertex<I>>,
    pub edges: Vec<Edge<I>>,
}

#[derive(Debug, Clone)]
pub struct Vertex<I> {
    pub id: I,
    pub neighbors: Vec<I>,
}

#[derive(Debug, Clone)]
pub struct Edge<I> {
    pub left: I,
    pub right: I,
}

impl<I> Graph<I> {
    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }
}

impl<I> Graph<I>
where
    I: FromStr + Eq + std::hash::Hash + Clone,
    I::Err: std::error::Error,
{
    pub fn load_file<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let content = fs::read_to_string(path)?;

        let mut vertices: HashMap<I, Vertex<I>> = HashMap::new();
        let mut edges: Vec<Edge<I>> = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut parts = line.split_whitespace();
            let left_id: I = parts.next()
                .ok_or_else(|| Error::BuildGroup(format!("invalid line '{}'", line)))?
                .parse()
                .map_err(|e| Error::BuildGroup(format!("parse invalid line '{}' for '{}'", line, e)))?;
            let right_id: I = parts.next()
                .ok_or_else(|| Error::BuildGroup(format!("invalid line '{}'", line)))?
                .parse()
                .map_err(|e| Error::BuildGroup(format!("parse invalid line '{}' for '{}'", line, e)))?;

            // 插入边
            edges.push(Edge {
                left: left_id.clone(),
                right: right_id.clone(),
            });

            // 更新邻接表
            vertices
                .entry(left_id.clone())
                .or_insert_with(|| Vertex {
                    id: left_id.clone(),
                    neighbors: vec![],
                })
                .neighbors
                .push(right_id.clone());

            vertices
                .entry(right_id.clone())
                .or_insert_with(|| Vertex {
                    id: right_id.clone(),
                    neighbors: vec![],
                })
                .neighbors
                .push(left_id);
        }

        Ok(Graph { vertices, edges })
    }
}
