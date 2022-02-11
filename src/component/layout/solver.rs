use std::collections::HashMap;

use crate::cassowary::*;

use crate::{error::LayoutEquationProblem, terminal::cursor::Index};

use super::child::Child;

pub type Variables = HashMap<Variable, (usize, ElementPart)>;

pub struct Solver {
    solver: cassowary::Solver,

    width: Variable,
    height: Variable,

    pub variables: Variables,
    elements: Vec<Element>,
}

pub struct Element {
    pub left_x: Variable,
    pub left_y: Variable,
    pub right_x: Variable,
    pub right_y: Variable,

    width: Variable,
    height: Variable,
}

#[derive(Debug)]
pub enum ElementPart {
    LeftX,
    LeftY,
    RightX,
    RightY,
}

impl Solver {
    pub fn new() -> Self {
        let mut res = Self::default();
        res.add_edit_size();
        res
    }

    pub fn add_edit_size(&mut self) {
        self.solver.add_edit_variable(self.width, STRONG).unwrap();
        self.solver.add_edit_variable(self.height, STRONG).unwrap();
    }

    pub fn get_changes(&mut self) -> (&[(Variable, f64)], &Variables) {
        (self.solver.fetch_changes(), &self.variables)
    }

    pub fn add_custom_constraint(
        &mut self,
        constraint: Constraint,
    ) -> Result<(), LayoutEquationProblem> {
        self.solver.add_constraint(constraint).map_err(LayoutEquationProblem::from)
    }

    pub fn merge_childs(&mut self, childs: &[Child]) {
        childs.iter().for_each(|c| self.add_child(c));
    }

    pub fn suggest_size(&mut self, width: Index, height: Index) {
        self.solver.suggest_value(self.width, width as f64).unwrap();
        self.solver.suggest_value(self.height, height as f64).unwrap();
    }

    pub fn element_len(&self) -> usize {
        self.elements.len()
    }

    pub fn clear(&mut self) {
        self.solver.reset();
        self.variables.clear();
        self.elements.clear();

        self.add_edit_size();
    }

    pub fn get(&self, index: usize) -> Option<&Element> {
        self.elements.get(index)
    }

    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }

    pub fn add_child(&mut self, child: &Child) {
        let element = Element::new();
        let index = self.elements.len();

        self.elements.push(element);

        self.add_variables(index);
        self.merge_size_from_child(child, index);
        self.add_default_constraint(index);
    }

    fn add_variables(&mut self, index: usize) {
        let element = self.elements.get(index).unwrap();

        self.variables.insert(element.left_x, (index, ElementPart::LeftX));
        self.variables.insert(element.left_y, (index, ElementPart::LeftY));
        self.variables.insert(element.right_x, (index, ElementPart::RightX));
        self.variables.insert(element.right_y, (index, ElementPart::RightY));

        self.solver.add_edit_variable(element.width, REQUIRED - 1.0).unwrap();
        self.solver.add_edit_variable(element.height, REQUIRED - 1.0).unwrap();
    }

    pub fn merge_size_from_child(&mut self, child: &Child, index: usize) {
        let element = self.elements.get(index).unwrap();

        self.solver.suggest_value(element.width, child.size().width() as f64).unwrap();
        self.solver.suggest_value(element.height, child.size().height() as f64).unwrap();
    }

    fn add_default_constraint(&mut self, index: usize) {
        if index == 0 {
            let element = self.get(index).unwrap();
            let start_x_from_zero = element.left_x | EQ(REQUIRED) | 0.0;
            let start_y_from_zero = element.left_y | EQ(REQUIRED) | 0.0;
            self.solver.add_constraints(&[start_x_from_zero, start_y_from_zero]).unwrap();
        }

        let element = self.get(index).unwrap();

        let non_negative_width = (element.right_x - element.left_x + 1.0) | GE(REQUIRED) | 0.0;
        let non_negative_height = (element.right_y - element.left_y + 1.0) | GE(REQUIRED) | 0.0;

        let left_x_positive = element.left_x | GE(REQUIRED) | 0.0;
        let left_y_positive = element.left_y | GE(REQUIRED) | 0.0;

        let right_x_lower_width = element.right_x | LE(REQUIRED) | (self.width - 1.0);
        let right_y_lower_height = element.right_y | LE(REQUIRED) | (self.height - 1.0);

        // First elements have priority
        let width_strength = MEDIUM - index as f64;

        let preffered_width =
            (element.right_x - element.left_x + 1.0) | EQ(width_strength) | element.width;
        let preffered_height =
            (element.right_y - element.left_y + 1.0) | EQ(width_strength) | element.height;

        self.solver
            .add_constraints(&[
                non_negative_width,
                non_negative_height,
                left_x_positive,
                left_y_positive,
                right_x_lower_width,
                right_y_lower_height,
                preffered_width,
                preffered_height,
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
            width: Variable::new(),
            height: Variable::new(),
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
