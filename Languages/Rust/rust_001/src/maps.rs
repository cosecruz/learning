pub fn learn_maps(){
  let mut map = std::collections::HashMap::new();

  let key = String::from("color");
    let value = String::from("blue");

    map.insert(key, value);


  for (k, v)in map.iter(){
    println!("{},{}", k, v);
  }
}
