use std::io;
fn is_vowel(ch : &char) -> bool{
    *ch == 'a' || *ch == 'e' || *ch == 'i' || *ch == 'o' || *ch == 'u' || *ch == 'A' || *ch == 'E' || *ch == 'I' || *ch == 'O' || *ch == 'U'
}
fn func(input : &str)-> String{
   let ch = input.trim().chars().next().unwrap();
   let input = input.trim();
   let mut soup = String::from(input);
   if is_vowel(&ch) {
    soup.push_str("-hay");
    return soup;
   } else{
      let mut flag = true;
      let mut s = String::new();
      let mut pig = String::new();
      pig.push('-');
      for c in soup.chars(){
         if !is_vowel(&c) && flag {
            pig.push(c);
            flag = false;
            continue;
         }
         s.push(c);
      }
      pig.push_str("ay");
      let st = s + &pig;
      st
   }
}
fn main() {
   let mut input = String::new();
   println!("Enter any word");
   io::stdin().read_line(&mut input).expect("There was an error.");
   let v : Vec<&str> = input.split(' ').collect();
   println!("Sentence in Pig-Latin would be -");
   let mut ffinal = String::new();
   for s in v {
      ffinal.push_str(func(s).as_str());
      ffinal.push_str(" ");
   }
   println!("{}",ffinal);
}

