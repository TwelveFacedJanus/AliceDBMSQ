use std::error;
use std::any::Any;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;


#[derive(Debug)]
pub struct AliceColumn<T> {
    data: Vec<T>,
    name: String,
}

pub struct AliceTable {
    name: String,
    columns: Vec<Box<dyn AnyColumn>>
}


trait AnyColumn: Any {
    fn get_name(&self) -> &str;
    fn len(&self) -> usize;
    // Добавляем метод для приведения к Any
    fn as_any(&self) -> &dyn Any;
    // И для mutable варианта
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static + std::fmt::Debug> AnyColumn for AliceColumn<T> {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl fmt::Debug for AliceTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AliceTable")
            .field("name", &self.name)
            .field("columns_count", &self.columns.len())
            .finish()
    }
}

impl fmt::Debug for dyn AnyColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AliceColumn")
            .field("name", &self.get_name())  // Use the trait method instead of field
            .field("length", &self.len())    // Add length for more info
            .finish()
    }
}

impl<T> AliceColumn<T> {
    pub fn new(name: &str) -> Self {
        let mut data = Vec::new();
        Self { data, name: name.to_string() }
    }

    pub fn get_all(&self) -> &Vec<T> {
        &self.data
    }
    
    pub fn get_all_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    pub fn insert(&mut self, value: T) -> Result<()> {
        self.data.push(value);
        Ok(())
    }

    pub fn get_by_index(&self, indx: usize) -> Result<Option<&T>> {
        if indx > self.get_size() {
            Ok(None)
        } else {
            Ok(Some(&self.data[indx]))
        }
    }

    pub fn get_mut_by_index(&mut self, indx: usize) -> Result<Option<&mut T>> {
        if indx > self.get_size() {
            Ok(None)
        } else {
            Ok(Some(&mut self.data[indx]))
        }
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }
}

impl AliceTable {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            columns: Vec::new(),
        }
    }

    pub fn add_column<T: 'static + std::fmt::Debug>(&mut self, column: AliceColumn<T>) {
        self.columns.push(Box::new(column));
    }

    pub fn get_column<T: 'static>(&self, name: &str) -> Option<&AliceColumn<T>> {
        for column in &self.columns {
            if column.get_name() == name {
                return column.as_any().downcast_ref::<AliceColumn<T>>();
            }
        }
        None
    }

    pub fn get_column_mut<T: 'static>(&mut self, name: &str) -> Option<&mut AliceColumn<T>> {
        for column in &mut self.columns {
            if column.get_name() == name {
                return column.as_any_mut().downcast_mut::<AliceColumn<T>>();
            }
        }
        None
    }
}

fn main() {
    let mut c1 = AliceColumn::<i32>::new("ID");
    let mut c2 = AliceColumn::<String>::new("Usernames");
    for i in 0..12 {
        c1.insert(i);
    }

    let mut table = AliceTable::new("Users");
   
    table.add_column(c1);
    table.add_column(c2);


    println!("{:#?}", table.columns[0]);
    let c3 = table.get_column::<i32>("ID");
    println!("{:#?}", c3);
}
