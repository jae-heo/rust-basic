pub fn test_lifetime() {
    let i: ImportantExcerpt;

    {
        let first_sentence  = String::from("test");
        i = ImportantExcerpt {
            part: &first_sentence,
            test: "test",
        };
    }
    
    
    //If this part is uncommented, this will make error because of its lifetime rules.
    // println!("test : {:?}", i.test);
}
#[derive(Debug)]
pub struct ImportantExcerpt<'a> {
    part: &'a str,
    test: &'a str,
}