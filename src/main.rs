fn request() -> String {
 let mut value: String = String::new();

 std::io::stdin().read_line(&mut value).expect("Input failed");

 value = value.trim().to_string();
 value = value.replace("\n", "") ;
 value = value.replace("\r", "") ;

 value
}//fn request() -> String {

fn main(){
 loop {
  println!("\r\n\r\nvector:"); 

  let mut text: String = request();

  if &text[..] == "exit" {
   break;   

  } else {//if &text[..] == "exit" {
   let mut integer: Vec<i32> = serde_json::from_str(&text[..]).expect("Wrong format");
   let mut amount : usize    = integer.len()                                         ;

   println!("\r\ntarget:"); 

   text = request();

   if &text[..] == "exit" {
    break;   

   } else {//if &text[..] == "exit" {
    let mut quadruplets: Vec<Vec<i32>> = vec![]                                                ;
    let     target     :         i32   = serde_json::from_str(&text[..]).expect("Wrong format");

    if amount > 3 {
     let m: usize = amount - 2;
     let n: usize = amount - 3;

     integer.sort();

     for i in 0..n {
      for j in (i + 1)..m {
       let mut forward = j + 1;
       let mut reverse = m + 1;

       while forward < reverse {
        let sum: i64 = integer[i] as i64 + integer[j] as i64 + integer[forward] as i64 + integer[reverse] as i64;

        if sum == target as i64 {
         quadruplets.push([integer[i], integer[j], integer[forward], integer[reverse]].to_vec());

         forward += 1;
         reverse -= 1;

        } else {//if sum == target as i64 {
         if sum < target as i64 {
          forward += 1;

         } else {//if sum < target as i64 {
          reverse -= 1;

         }//} else {//if sum < target as i64 {
        }//} else {//if sum == target as i64 {
       }//while forward < reverse {
      }//for j in (i + 1)..m {
     }//for i in 0..n {
    }//if amount > 3 {

    quadruplets.sort();

    amount = quadruplets.len();

    if amount > 1 {
     let last: usize = amount - 1usize;

     for i in (0..last).rev() {
      let j: usize = i + 1usize;

      if quadruplets[i][0] == quadruplets[j][0] && quadruplets[i][1] == quadruplets[j][1] && quadruplets[i][2] == quadruplets[j][2] && quadruplets[i][3] == quadruplets[j][3] {
       quadruplets.remove(j);

      }//if quadruplets[i][0] == quadruplets[j][0] && quadruplets[i][1] == quadruplets[j][1] && quadruplets[i][2] == quadruplets[j][2] && quadruplets[i][3] == quadruplets[j][3] {
     }//for i in (0..last).rev() {
    }//if amount > 1 {

    println!("\r\nquadruplets:\r\n{:?}", quadruplets);
   }//} else {//if &text[..] == "exit" {
  }//} else {//if &text[..] == "exit" {
 }//loop {
}//fn main(){
