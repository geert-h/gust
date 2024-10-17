use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};

trait ComponentVec {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn push_none(&mut self);
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None)
    }
}

pub struct NewComponentStorage {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl NewComponentStorage {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }

        self.entities_count += 1;
        entity_id
    }

    pub fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                component_vec.borrow_mut()[entity] = Some(component);
                return;
            }
        }

        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(component);
        self.component_vecs
            .push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn borrow_component_vec<ComponentType: 'static>(
        &self,
    ) -> Option<Ref<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(component_vec.borrow());
            }
        }
        None
    }

    pub fn borrow_mut_component_vec<ComponentType: 'static>(
        &self,
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }

    pub fn get_entities_count(&self) -> usize {
        self.entities_count
    }

    // Method to query immutable references to components
    pub fn query_components<'a, C1: 'static, C2: 'static>(
        &'a self,
    ) -> Option<QueryComponents<'a, C1, C2>> {
        let comp_vec1 = self.borrow_component_vec::<C1>()?;
        let comp_vec2 = self.borrow_component_vec::<C2>()?;

        Some(QueryComponents::new(comp_vec1, comp_vec2))
    }

    // Method to query mutable references to components
    pub fn query_components_mut<'a, C1: 'static, C2: 'static>(
        &'a self,
    ) -> Option<QueryComponentsMut<'a, C1, C2>> {
        let comp_vec1 = self.borrow_mut_component_vec::<C1>()?;
        let comp_vec2 = self.borrow_mut_component_vec::<C2>()?;

        Some(QueryComponentsMut::new(comp_vec1, comp_vec2))
    }
}

// Struct for immutable component queries
pub struct QueryComponents<'comp, C1: 'static, C2: 'static> {
    comp_vec1: Ref<'comp, Vec<Option<C1>>>,
    comp_vec2: Ref<'comp, Vec<Option<C2>>>,
    index: usize,
}

impl<'comp, C1: 'static, C2: 'static> QueryComponents<'comp, C1, C2> {
    fn new(
        comp_vec1: Ref<'comp, Vec<Option<C1>>>,
        comp_vec2: Ref<'comp, Vec<Option<C2>>>,
    ) -> Self {
        Self {
            comp_vec1,
            comp_vec2,
            index: 0,
        }
    }
}

impl<'comp, C1: 'static, C2: 'static> Iterator for QueryComponents<'comp, C1, C2> {
    type Item = (Option<&'comp C1>, Option<&'comp C2>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.comp_vec1.len() && self.index < self.comp_vec2.len() {
            let c1_ref = self.comp_vec1[self.index].as_ref();
            let c2_ref = self.comp_vec2[self.index].as_ref();
            self.index += 1;
            Some((c1_ref, c2_ref))
        } else {
            None
        }
    }
}

// Struct for mutable component queries
pub struct QueryComponentsMut<'comp, C1: 'static, C2: 'static> {
    comp_vec1: RefMut<'comp, Vec<Option<C1>>>,
    comp_vec2: RefMut<'comp, Vec<Option<C2>>>,
    index: usize,
}

impl<'comp, C1: 'static, C2: 'static> QueryComponentsMut<'comp, C1, C2> {
    fn new(
        comp_vec1: RefMut<'comp, Vec<Option<C1>>>,
        comp_vec2: RefMut<'comp, Vec<Option<C2>>>,
    ) -> Self {
        Self {
            comp_vec1,
            comp_vec2,
            index: 0,
        }
    }
}

impl<'comp, C1: 'static, C2: 'static> Iterator for QueryComponentsMut<'comp, C1, C2> {
    type Item = (Option<&'comp mut C1>, Option<&'comp mut C2>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.comp_vec1.len() && self.index < self.comp_vec2.len() {
            // Get mutable references to the Option<T> elements
            let c1_option = &mut self.comp_vec1[self.index];
            let c2_option = &mut self.comp_vec2[self.index];
            self.index += 1;

            Some((c1_option.as_mut(), c2_option.as_mut()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Health(i32);

    struct Name(&'static str);

    #[test]
    fn test_query_components() {
        let mut storage = NewComponentStorage::new();
        let icarus_entity = storage.new_entity();
        storage.add_component_to_entity(icarus_entity, Name("Icarus"));
        storage.add_component_to_entity(icarus_entity, Health(-10));

        let prometheus_entity = storage.new_entity();
        storage.add_component_to_entity(prometheus_entity, Name("Prometheus"));
        storage.add_component_to_entity(prometheus_entity, Health(100));

        let zeus_entity = storage.new_entity();
        storage.add_component_to_entity(zeus_entity, Name("Zeus"));

        // Immutable query
        if let Some(query) = storage.query_components::<Health, Name>() {
            for (health_opt, name_opt) in query {
                if let (Some(health), Some(name)) = (health_opt, name_opt) {
                    if health.0 < 0 {
                        println!("{} has perished", name.0);
                    }
                }
            }
        }

        // Mutable query
        if let Some(query_mut) = storage.query_components_mut::<Health, Name>() {
            for (health_opt, name_opt) in query_mut {
                if let (Some(health), Some(name)) = (health_opt, name_opt) {
                    if name.0 == "Icarus" && health.0 <= 0 {
                        health.0 = 100;
                        println!("{} has been resurrected", name.0);
                    }
                }
            }
        }

        // Verify the resurrection
        if let Some(query) = storage.query_components::<Health, Name>() {
            for (health_opt, name_opt) in query {
                if let (Some(health), Some(name)) = (health_opt, name_opt) {
                    if name.0 == "Icarus" {
                        assert_eq!(health.0, 100);
                    }
                }
            }
        };
    }
}