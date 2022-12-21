use core::panic;

#[doc = include_str!("../README.md")]

#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub enum StringDiffOpKind {
    Substitute(char, char),  
    Insert(char),
    Delete(char),
    Transpose(usize, usize),
}

#[derive(Debug)]
pub struct StringDiffOp {
    pub kind: StringDiffOpKind,
    pub index: isize,
}

pub (crate) fn remove(start: usize, stop: usize, s: &str) -> String {
    let mut rslt = "".to_string();
    for (i, c) in s.chars().enumerate() {
        if start > i || stop < i + 1 {
            rslt.push(c);
        }
    }
    rslt
}

pub fn applyDiff(s: &str, diffs: Vec<StringDiffOp>) -> String {
    let mut new_string : String = s.into();
    
    for i in diffs.iter(){
        match i.kind {
            StringDiffOpKind::Delete(x) => {
                new_string = remove(
                        i.index as usize, 
                    (i.index as usize)+1 as usize,
                     &new_string);

            },
            StringDiffOpKind::Insert(x) => {
                new_string.push(x);
            },
            StringDiffOpKind::Substitute(x,y) => {
                new_string.replace_range(
                    (i.index as usize)..( (i.index as usize)+1 as usize),
                &y.to_string())
            },
            _ => {
                panic!("Unrecoginzed Opperation")
            }
        }
    }
    return new_string.clone();
}

pub trait StringDiffAlgorithm {
    //fn diff<'a>(s1: &'a str, s2: &'a str) -> Vec<StringDiffOp>;
    fn diff<'a>(&self, s1: &'a str, s2: &'a str) -> Vec<StringDiffOp>;
    fn distance<'a>(&self, s1: &'a str, s2: &'a str) -> usize;
}

pub struct HammingDistance {}
impl HammingDistance{}
impl StringDiffAlgorithm for HammingDistance {
    fn diff<'a>(&self, s1: &'a str, s2: &'a str) -> Vec<StringDiffOp>{
        let mut opp_vec : Vec<StringDiffOp> = Vec::new();
        if s1.len() != s2.len(){
            panic!("Strings must be same length");
        }
        else{
            for i in 0..s1.len(){
                if s1.chars().nth(i).unwrap() != s2.chars().nth(i).unwrap(){
                    let new_opp = StringDiffOp{
                        kind : StringDiffOpKind::Substitute(
                            s1.chars().nth(i).unwrap(),
                            s2.chars().nth(i).unwrap() ),
                        index : i as isize
                    };
                    opp_vec.push(new_opp)
                }
            }
        }
        return opp_vec;
    }        

    
    fn distance<'a>(&self, s1: &'a str, s2: &'a str) -> usize{
        let mut edit_distance = 0;
        if s1.len() != s2.len(){
            panic!("Strings must be same length");
        }
        else{
            for i in 0..s1.len(){
                if s1.chars().nth(i).unwrap() != s2.chars().nth(i).unwrap(){
                    edit_distance += 1;
                }
            }
        }
        return edit_distance;
    }
}


pub struct LevenshteinDistance {}
impl LevenshteinDistance{
    pub(crate) fn min_dist(x: usize, y: usize, z: usize) -> usize {
        if x <= y && x <= z{
            return x;
        }
        else if y <= x && y <= z {
            return y;
        }
        else{
            return z;
        }
    }

    pub(crate) fn min_dist_with_dir(x: usize, y: usize, z: usize) -> (usize,char){
        if x <= y && x <= z{
            return (x, '^');
        }
        if y <= x && y <= z {
            return (y, '<');
        }
        if z <= x && z <= y {
            return (z, '\\');
        }
        return (0, 'x')
    }

    pub(crate) fn print_vector<T: std::fmt::Debug>(my_vector: &[T]){
        for i in my_vector.iter(){
            println!("{:?}", i);
        }
    }

    pub(crate) fn get_operations(my_opp: &Vec<Vec<char>>, 
                                    left_string: &str,
                                    top_string: &str) -> Vec<StringDiffOp> {
                        
        let mut my_opp_vector : Vec<StringDiffOp> = Vec::new();
        let mut top_str_len = top_string.len();
        let mut left_str_len = left_string.len();
        let mut prev_char : char = ' ';
        
        
        loop{
            
            if top_str_len == 0 && left_str_len == 0{
                break;
            }
                        //Rows               Columns
            match my_opp[left_str_len][top_str_len] {
                //insertion
                '^' => {
                    let insertionOpp : StringDiffOp = StringDiffOp{
                        kind : StringDiffOpKind::Insert(left_string.chars().nth(left_str_len-1).unwrap()),
                        index : -1
                    };
                    left_str_len -= 1;
                    my_opp_vector.push(insertionOpp);
                    prev_char = '^';

                },
                //substitution
                '\\' => {
                    if prev_char == '^'{
                        my_opp_vector.reverse();
                    }
 
                    if left_string.chars().nth(left_str_len-1).unwrap() != top_string.chars().nth(top_str_len-1).unwrap(){
                        let insertionOpp : StringDiffOp = StringDiffOp{
                            kind : StringDiffOpKind::Substitute(
                                top_string.chars().nth(top_str_len-1).unwrap(),
                                left_string.chars().nth(left_str_len-1).unwrap()
                                
                            ),
                            index : (top_str_len as isize) - 1 
                        };                       
                        my_opp_vector.push(insertionOpp);     
                    }                
                    left_str_len -= 1;
                    top_str_len -= 1; 
                    prev_char = '\\';
                },
                //deletion
                '<' => {
                    if prev_char == '^'{
                        my_opp_vector.reverse();
                    }
                    let insertionOpp : StringDiffOp = StringDiffOp{
                        kind : StringDiffOpKind::Delete(top_string.chars().nth(top_str_len-1).unwrap()),
                        index : (top_str_len as isize) - 1
                    };
                    top_str_len -= 1;
                    my_opp_vector.push(insertionOpp);      
                    prev_char = '<';
                },
                _ => { 
                    panic!("UNRECOGINZED SYMBOL OPERATION !")
                }
            }

        }
        return my_opp_vector;
        //println!("{:?}" , my_opp_vector)
    }
   
}
impl StringDiffAlgorithm for LevenshteinDistance {
    //fn diff<'a>(s1: &'a str, s2: &'a str) -> Vec<StringDiffOp>{}
    fn diff<'a>(&self, s1: &'a str, s2: &'a str) -> Vec<StringDiffOp> {
        
        let first_string_len : usize = s1.len();
        let second_string_len : usize = s2.len();
        
        let mut dist_vector = vec![vec![0usize ; second_string_len + 1]; first_string_len + 1];
        let mut dir_vector: Vec<Vec<char>> = vec![vec![' ' ; second_string_len + 1]; first_string_len + 1];

        for i in 0..first_string_len+1{
            dist_vector[i][0] = i;
        }
        for j in 0..second_string_len+1{
            dist_vector[0][j] = j;
        }

        dir_vector[0][0] = '\\';
        for j in 1..second_string_len+1{
            dir_vector[0][j] = '<';
        }
        for i in 1..first_string_len+1{
            dir_vector[i][0] = '^';
        }

        let mut sub_cost : usize = 0;
        for i in 1..first_string_len+1{
            for j in 1..second_string_len+1{
                if s1.chars().nth(i-1).unwrap() ==  s2.chars().nth(j-1).unwrap(){
                    sub_cost = 0;
                }
                else{
                    sub_cost = 1;
                }
                (dist_vector[i][j], dir_vector[i][j])  = LevenshteinDistance::min_dist_with_dir(
                    dist_vector[i-1][j] + 1, //deletion
                    dist_vector[i][j-1] + 1, //insertion
                    dist_vector[i-1][j-1] + sub_cost); //substitution
            }
        }

        //LevenshteinDistance::print_vector(&dist_vector);
        //LevenshteinDistance::print_vector(&dir_vector);
        /*
        for row in dist_vector.iter(){
            println!("{:?}", row)
        }
        for row in dir_vector.iter(){
            println!("{:?}", row)
        }
        */

        //println!("Made it here");
        let test = LevenshteinDistance::get_operations(&dir_vector, s1, s2);
        println!("{:?}", test);
        //return dist_vector[first_string_len][second_string_len];
        return test;
    }
    fn distance<'a>(&self, s1: &'a str, s2: &'a str) -> usize {
        
        let first_string_len : usize = s1.len();
        let second_string_len : usize = s2.len();
        
        let mut dist_vector = vec![vec![0usize ; second_string_len + 1]; first_string_len + 1];

        for i in 0..first_string_len+1{
            dist_vector[i][0] = i;
        }

        for j in 0..second_string_len+1{
            dist_vector[0][j] = j;
        }

        let mut sub_cost : usize = 0;
        for i in 1..first_string_len+1{
            for j in 1..second_string_len+1{
                if s1.chars().nth(i-1).unwrap() ==  s2.chars().nth(j-1).unwrap(){
                    sub_cost = 0;
                }
                else{
                    sub_cost = 1;
                }
                dist_vector[i][j] = LevenshteinDistance::min_dist(
                        dist_vector[i-1][j] + 1,
                        dist_vector[i][j-1] + 1,
                        dist_vector[i-1][j-1] + sub_cost);
            }
        }
        println!("{:?}" , dist_vector);
        return dist_vector[first_string_len][second_string_len];
    }
}


#[cfg(test)]
mod dcode_tests{

    fn eq_with_nan_eq(a: &StringDiffOp, b: &StringDiffOp) -> bool {
        if (a.index == b.index) && (b.kind == a.kind){
            return true;
        }
        return false;
    }
    fn vec_compare(va: &Vec<StringDiffOp>, vb: &Vec<StringDiffOp>) -> bool {
        (va.len() == vb.len()) &&  // zip stops at the shortest
        va.iter()
        .zip(vb)
        .all(|(a,b)| eq_with_nan_eq(a,b))
    }


    use crate::{StringDiffAlgorithm, StringDiffOp, HammingDistance};

    #[test]
    fn test_LevenshteinDistance_edit_distance(){
        let test_struct = super::LevenshteinDistance{};

        assert_eq!(3 , test_struct.distance("sets", "reset"));
        assert_eq!(3 , test_struct.distance("sitting", "kitten"));
        assert_eq!(3 , test_struct.distance("Sunday", "Saturday"));
    }

    #[test]
    fn test_LevenshteinDistance_opp_distance(){
        let test_struct = super::LevenshteinDistance{};

        let mut test_vec: Vec<StringDiffOp> = Vec::new();
        test_vec.push(StringDiffOp {
            kind: super::StringDiffOpKind::Insert('g'),
            index: -1
        });
        test_vec.push(StringDiffOp {
            kind: super::StringDiffOpKind::Substitute('e','i'),
            index: 4
        });
        test_vec.push(StringDiffOp {
            kind: super::StringDiffOpKind::Substitute('k','s'),
            index: 0
        });

        let mut test_vec_2: Vec<StringDiffOp> = Vec::new();
        test_vec_2.push(StringDiffOp {
            kind: super::StringDiffOpKind::Substitute('r','n'),
            index: 4
        });
        test_vec_2.push(StringDiffOp {
            kind: super::StringDiffOpKind::Delete('t'),
            index: 2
        });
        test_vec_2.push(StringDiffOp {
            kind: super::StringDiffOpKind::Delete('a'),
            index: 1
        });

        let mut test_vec_3: Vec<StringDiffOp> = Vec::new();
        test_vec_3.push(StringDiffOp {
            kind: super::StringDiffOpKind::Insert('S'),
            index: -1
        });
        test_vec_3.push(StringDiffOp {
            kind: super::StringDiffOpKind::Delete('E'),
            index: 1
        });
        test_vec_3.push(StringDiffOp {
            kind: super::StringDiffOpKind::Delete('R'),
            index: 0
        });

        assert_eq!(vec_compare(&test_vec, &test_struct.diff("sitting", "kitten")) , true );
        assert_eq!(vec_compare(&test_vec_2, &test_struct.diff("Sunday", "Saturday")) , true );
        assert_eq!(vec_compare(&test_vec_3, &test_struct.diff("SETS", "RESET")) , true );
    }

    #[test]
    fn test_HammingDistance_edit_distance(){
        let test_struct = super::HammingDistance{};

        assert_eq!(3 , test_struct.distance("karolin", "kathrin"));
        assert_eq!(3 , test_struct.distance("karolin", "kerstin"));
        assert_eq!(4 , test_struct.distance("kathrin", "kerstin"));
        assert_eq!(4 , test_struct.distance("0000", "1111"));
        assert_eq!(3 , test_struct.distance("2173896", "2233796"));
    }

    #[test] 
    fn test_HammingDistance_opp_distance(){
        let test_struct = super::HammingDistance{};

        let mut test_vec: Vec<StringDiffOp> = Vec::new();
        test_vec.push(
                super::StringDiffOp{
                    kind : super::StringDiffOpKind::Substitute('r', 't'),
                    index : 2
            });
        test_vec.push(
                super::StringDiffOp{
                    kind : super::StringDiffOpKind::Substitute('o', 'h'),
                    index : 3
            });
        test_vec.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('l', 'r'),
                index : 4
        });

        let mut test_vec_2: Vec<StringDiffOp> = Vec::new();
        test_vec_2.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('a', 'e'),
                index : 1
            });
        test_vec_2.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('o', 's'),
                index : 3
        });
        test_vec_2.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('l', 't'),
                index : 4
        });           

        let mut test_vec_3: Vec<StringDiffOp> = Vec::new();
        test_vec_3.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('a', 'e'),
                index : 1
            });
            test_vec_3.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('t', 'r'),
                index : 2
        });
        test_vec_3.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('h', 's'),
                index : 3
        });           
        test_vec_3.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('r', 't'),
                index : 4
        });        

        let mut test_vec_4: Vec<StringDiffOp> = Vec::new();
        test_vec_4.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('0', '1'),
                index : 0
        });
        test_vec_4.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('0', '1'),
                index : 1
        });
        test_vec_4.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('0', '1'),
                index : 2
        });           
        test_vec_4.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('0', '1'),
                index : 3
        });


        let mut test_vec_5: Vec<StringDiffOp> = Vec::new();
        test_vec_5.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('1', '2'),
                index : 1
        });
        test_vec_5.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('7', '3'),
                index : 2
        });
        test_vec_5.push(
            super::StringDiffOp{
                kind : super::StringDiffOpKind::Substitute('8', '7'),
                index : 4
        });           


        assert_eq!(vec_compare(&test_vec, &test_struct.diff("karolin", "kathrin")) , true );
        assert_eq!(vec_compare(&test_vec_2, &test_struct.diff("karolin", "kerstin")) , true );
        assert_eq!(vec_compare(&test_vec_3, &test_struct.diff("kathrin", "kerstin")) , true );
        assert_eq!(vec_compare(&test_vec_4, &test_struct.diff("0000", "1111")) , true );
        assert_eq!(vec_compare(&test_vec_5, &test_struct.diff("2173896", "2233796")) , true );

        //assert_eq!( (test_vec_5[0].kind).0 ,super::StringDiffOpKind::Substitute('7', '3'));

    }

    #[test]
    fn test_apply_diffs(){

        let mut test_vec: Vec<StringDiffOp> = Vec::new();
        test_vec.push(StringDiffOp {
            kind: super::StringDiffOpKind::Insert('g'),
            index: -1
        });
        test_vec.push(StringDiffOp {
            kind: super::StringDiffOpKind::Substitute('e','i'),
            index: 4
        });
        test_vec.push(StringDiffOp {
            kind: super::StringDiffOpKind::Substitute('k','s'),
            index: 0
        });        

        let mut test_vec_2: Vec<StringDiffOp> = Vec::new();
        test_vec_2.push(StringDiffOp {
            kind: super::StringDiffOpKind::Substitute('r','n'),
            index: 4
        });
        test_vec_2.push(StringDiffOp {
            kind: super::StringDiffOpKind::Delete('t'),
            index: 2
        });
        test_vec_2.push(StringDiffOp {
            kind: super::StringDiffOpKind::Delete('a'),
            index: 1
        });

        let mut test_vec_3: Vec<StringDiffOp> = Vec::new();
        test_vec_3.push(StringDiffOp {
            kind: super::StringDiffOpKind::Insert('S'),
            index: -1
        });
        test_vec_3.push(StringDiffOp {
            kind: super::StringDiffOpKind::Delete('E'),
            index: 1
        });
        test_vec_3.push(StringDiffOp {
            kind: super::StringDiffOpKind::Delete('R'),
            index: 0
        });

  

        assert_eq!(String::from("sitting") , super::applyDiff("kitten", test_vec)   );       
        assert_eq!(String::from("Sunday") , super::applyDiff("Saturday", test_vec_2)   );
        assert_eq!(String::from("SETS"), super::applyDiff("RESET", test_vec_3)  );
    }
    

}
