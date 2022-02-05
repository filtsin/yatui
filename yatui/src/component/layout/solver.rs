use std::collections::HashMap;

use crate::cassowary::*;

use crate::{
    component::size_hint::SizeHint, error::LayoutEquationProblem, terminal::cursor::Index,
};

use super::child::Child;

pub struct Solver {
    solver: cassowary::Solver,

    width: Variable,
    height: Variable,

    pub variables: HashMap<Variable, (usize, &'static str)>,
    elements: Vec<Element>,
}

pub struct Element {
    pub left_x: Variable,
    pub left_y: Variable,
    pub right_x: Variable,
    pub right_y: Variable,

    min_width: Variable,
    max_width: Variable,
    min_height: Variable,
    max_height: Variable,
}

impl Solver {
    pub(crate) fn new() -> Self {
        let mut res = Self::default();
        res.add_edit_size();
        res
    }

    pub(crate) fn add_edit_size(&mut self) {
        self.solver.add_edit_variable(self.width, STRONG).unwrap();
        self.solver.add_edit_variable(self.height, STRONG).unwrap();
    }

    pub(crate) fn get_changes(&mut self) -> &[(Variable, f64)] {
        self.solver.fetch_changes()
    }

    pub(crate) fn add_custom_constraint(
        &mut self,
        constraint: Constraint,
    ) -> Result<(), LayoutEquationProblem> {
        self.solver.add_constraint(constraint).map_err(LayoutEquationProblem::from)
    }

    pub(crate) fn merge_childs(&mut self, childs: &[Child]) {
        childs.iter().for_each(|c| self.add_child(c));
    }

    pub(crate) fn suggest_size(&mut self, width: Index, height: Index) {
        self.solver.suggest_value(self.width, width as f64).unwrap();
        self.solver.suggest_value(self.height, height as f64).unwrap();
    }

    pub(crate) fn element_len(&self) -> usize {
        self.elements.len()
    }

    pub(crate) fn clear(&mut self) {
        self.solver.reset();
        self.variables.clear();
        // TODO: Add width, height edit variables
    }

    pub(crate) fn get(&self, index: usize) -> Option<&Element> {
        self.elements.get(index)
    }

    pub(crate) fn add_child(&mut self, child: &Child) {
        let element = Element::new();
        let index = self.elements.len();

        self.elements.push(element);

        self.add_variables(index);
        self.merge_size_from_child(child, index);
        self.add_default_constraint(index);
    }

    pub(crate) fn add_variables(&mut self, index: usize) {
        let element = self.elements.get(index).unwrap();

        self.variables.insert(element.left_x, (index, "left_x"));
        self.variables.insert(element.left_y, (index, "left_y"));
        self.variables.insert(element.right_x, (index, "right_x"));
        self.variables.insert(element.right_y, (index, "right_y"));

        self.solver.add_edit_variable(element.min_width, STRONG).unwrap();
        self.solver.add_edit_variable(element.max_width, STRONG).unwrap();
        self.solver.add_edit_variable(element.min_height, STRONG).unwrap();
        self.solver.add_edit_variable(element.max_height, STRONG).unwrap();
    }

    pub(crate) fn merge_size_from_child(&mut self, child: &Child, index: usize) {
        let min_size = child.size().minimum();
        let max_size = child.size().maximum();

        let element = self.elements.get(index).unwrap();

        self.solver.suggest_value(element.min_width, min_size.width() as f64).unwrap();
        self.solver.suggest_value(element.max_width, max_size.width() as f64).unwrap();
        self.solver.suggest_value(element.min_height, min_size.height() as f64).unwrap();
        self.solver.suggest_value(element.max_height, max_size.height() as f64).unwrap();
    }

    pub(crate) fn add_default_constraint(&mut self, index: usize) {
        let element = self.get(index).unwrap();

        let non_negative_width = (element.right_x - element.left_x) | GE(REQUIRED) | 0.0;
        let non_negative_height = (element.right_y - element.left_y) | GE(REQUIRED) | 0.0;

        let left_x_positive = element.left_x | GE(REQUIRED) | 0.0;
        let left_y_positive = element.left_y | GE(REQUIRED) | 0.0;

        let right_x_lower_width = element.right_x | LE(REQUIRED) | self.width;
        let right_y_lower_height = element.right_y | LE(REQUIRED) | self.height;

        let preffered_min_width = (element.right_x - element.left_x) | GE(WEAK) | element.min_width;
        let preffered_min_height =
            (element.right_y - element.left_y) | GE(WEAK) | element.min_height;
        let preffered_max_width = (element.right_x - element.left_x) | LE(WEAK) | element.max_width;
        let preffered_max_height =
            (element.right_y - element.left_y) | LE(WEAK) | element.max_height;

        self.solver
            .add_constraints(&[
                non_negative_width,
                non_negative_height,
                left_x_positive,
                left_y_positive,
                right_x_lower_width,
                right_y_lower_height,
                preffered_min_width,
                preffered_max_width,
                preffered_min_height,
                preffered_max_height,
            ])
            .unwrap();
    }
}

impl Element {
    fn new() -> Self {
        Self {
            left_x: Variable::new(),
            left_y: Variable::new(),
            right_x: Variable::new(),
            right_y: Variable::new(),
            min_width: Variable::new(),
            max_width: Variable::new(),
            min_height: Variable::new(),
            max_height: Variable::new(),
        }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            solver: cassowary::Solver::new(),
            width: Variable::new(),
            height: Variable::new(),
            variables: HashMap::new(),
            elements: vec![],
        }
    }
}
