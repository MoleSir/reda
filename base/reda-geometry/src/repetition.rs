use crate::{base::Vec2, shape::CoordType};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct RegularRepetition<T> 
where 
    T: CoordType
{
    n_dimention: usize,
    m_dimention: usize,
    n_displayment: Vec2<T>,
    m_displayment: Vec2<T>,
}

impl<T: CoordType> RegularRepetition<T> {
    pub fn new(
        n_dimention: usize,
        m_dimention: usize,
        n_displayment: impl Into<Vec2<T>>,
        m_displayment: impl Into<Vec2<T>>,
    ) -> Self {
        Self { 
            n_dimention, 
            m_dimention, 
            n_displayment: n_displayment.into(), 
            m_displayment: m_displayment.into(), 
        }
    }

    pub fn new_rectilinear(x_dimention: usize, y_dimention: usize, x_spacing: T, y_spacing: T) -> Self {
        Self::new(
            x_dimention,
            y_dimention,
            Vec2::new(x_spacing, T::zero()),
            Vec2::new(T::zero(), y_spacing),
        )
    }

    pub fn iter(&self) -> impl Iterator<Item = Vec2<T>> {
        let mut current = Vec2::zero();
        (0..self.m_dimention).flat_map(move |_| {
            let mut row = current;
            current = current + self.m_displayment;
            
            (0..self.n_dimention).map(move |_| {
                let pos = row;
                row = row + self.n_displayment;
                pos
            })
        })
    }

    pub fn len(&self) -> usize {
        self.n_dimention * self.m_dimention
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0 
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct IrregularRepetition<T>
where
    T: CoordType,
{
    offsets: Vec<Vec2<T>>,
}

impl<T> IrregularRepetition<T>
where
    T: CoordType,
{
    pub fn new(offsets: impl Into<Vec<Vec2<T>>>) -> Self {
        IrregularRepetition { offsets: offsets.into() }
    }

    pub fn iter(&self) -> impl Iterator<Item = Vec2<T>> + '_ {
        self.offsets.iter().copied()
    }

    pub fn len(&self) -> usize {
        self.offsets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }
}
