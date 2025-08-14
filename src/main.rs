pub mod alice;
use alice::*;




fn main() {
    let mut database = AliceDatabase::new("Kingdom-System");
    database.create_table("Users");
    let mut table = database.get_mut_table("Users").unwrap().unwrap();
    let mut usernames_column = AliceColumn::<String>::new("Usernames");
    let mut id_column = AliceColumn::<usize>::new("ID");
    for i in 0..24 {
        id_column.insert(i);
    }
    table.add_column(id_column);
    table.add_column(usernames_column);
    



    println!("{:#?}", table.get_column::<usize>("ID").unwrap());


}
