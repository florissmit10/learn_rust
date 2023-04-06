use std::error::Error;
use std::str::FromStr;
use std::string::ParseError;

use serde::{Serialize};

#[derive(Debug, Default, Serialize, Clone)]
pub struct TodoList{
    items: Vec<TodoItem>
}

#[derive(Debug, Serialize, Clone)]
pub struct TodoItem{
    item: String,
    checked: bool
}

impl FromStr for TodoItem {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TodoItem{item:s.to_string(), checked:false })
    }
}

impl TodoItem {
    fn toggle(&mut self) {
        self.checked = !self.checked;
    }
}

impl TodoList {
    pub fn add(&mut self, todo: TodoItem) {
        self.items.push(todo)
    }

    pub fn toggle(&mut self, number: usize) -> Result<bool, Box<dyn Error>>{
        let Some(item) = self.items.get_mut(number) else {
            Err("no item found with number")?
        };
        item.toggle();
        Ok(item.checked)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo() {
        let mut todo_list = TodoList::default();
        assert_eq!(0, todo_list.items.len());

        todo_list.add(TodoItem::from_str("homework").unwrap());
        assert_eq!(1, todo_list.items.len());

        todo_list.add(TodoItem::from_str("cleaning").unwrap());
        assert_eq!(2, todo_list.items.len());

        assert_eq!("homework", todo_list.items.get(0).unwrap().item);
        assert_eq!("cleaning", todo_list.items.get(1).unwrap().item);

    }

    #[test]
    fn test_toggle_unkown() {
        let mut todo_list = TodoList::default();

        todo_list.add(TodoItem::from_str("homework").unwrap());
        todo_list.add(TodoItem::from_str("cleaning").unwrap());
        todo_list.add(TodoItem::from_str("cooking").unwrap());

        let result_error = todo_list.toggle(3);

        assert!(result_error.is_err());
        // All unchecked
        assert!(todo_list.items.iter().all(|i| i.checked == false));

    }

    #[test]
    fn test_empty_todo() {
        let mut todo_list = TodoList::default();

        todo_list.add(TodoItem::from_str("").unwrap());
        assert_eq!(1, todo_list.items.len());
        // All unchecked
        assert!(todo_list.items.iter().all(|i| i.checked == false));

    }

    #[test]
    fn test_toggle() {
        let mut todo_list = TodoList::default();
        todo_list.add(TodoItem::from_str("homework").unwrap());
        todo_list.add(TodoItem::from_str("cleaning").unwrap());
        todo_list.add(TodoItem::from_str("cooking").unwrap());
        // All unchecked
        assert!(todo_list.items.iter().all(|i| i.checked == false));

        todo_list.toggle(1).unwrap();
        assert_eq!(false, todo_list.items.get(0).unwrap().checked);
        assert_eq!(true, todo_list.items.get(1).unwrap().checked);
        assert_eq!(false, todo_list.items.get(2).unwrap().checked);

        todo_list.toggle(0).unwrap();
        assert_eq!(true, todo_list.items.get(0).unwrap().checked);
        assert_eq!(true, todo_list.items.get(1).unwrap().checked);
        assert_eq!(false, todo_list.items.get(2).unwrap().checked);

        todo_list.toggle(2).unwrap();
        assert_eq!(true, todo_list.items.get(0).unwrap().checked);
        assert_eq!(true, todo_list.items.get(1).unwrap().checked);
        assert_eq!(true, todo_list.items.get(2).unwrap().checked);

    }
}