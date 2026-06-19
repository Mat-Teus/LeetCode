fn main() {
    let vec = vec![4,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10];
    let s = String::from("bbbcccdddaaa");
    println!("{:?}", number_of_lines(vec, s));
}

pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    let mut solution = Vec::new();
    let mut lines = 0;
    let mut wide = 0;

    for i in s.chars(){
        if i == 'a'{
            if wide + widths[0] <= 100{
                wide+=widths[0];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[0];
            }
        }else if i == 'b'{
            if wide + widths[1] <= 100{
                wide+=widths[1];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[1];
            }
        }else if i == 'c'{
            if wide + widths[2] <= 100{
                wide+=widths[2];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[2];
            }
        }else if i == 'd'{
            if wide + widths[3] <= 100{
                wide+=widths[3];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[3];
            }
        }else if i == 'e'{
            if wide + widths[4] <= 100{
                wide+=widths[4];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[4];
            }
        }else if i == 'f'{
            if wide + widths[5] <= 100{
                wide+=widths[5];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[5];
            }
        }else if i == 'g'{
            if wide + widths[6] <= 100{
                wide+=widths[6];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[6];
            }
        }else if i == 'h'{
            if wide + widths[7] <= 100{
                wide+=widths[7];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[7];
            }
        }else if i == 'i'{
            if wide + widths[8] <= 100{
                wide+=widths[8];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[8];
            }
        }else if i == 'j'{
            if wide + widths[9] <= 100{
                wide+=widths[9];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[9];
            }
        }else if i == 'k'{
            if wide + widths[10] <= 100{
                wide+=widths[10];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[10];
            }
        }else if i == 'l'{
            if wide + widths[11] <= 100{
                wide+=widths[11];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[11];
            }
        }else if i == 'm'{
            if wide + widths[12] <= 100{
                wide+=widths[12];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[12];
            }
        }else if i == 'n'{
            if wide + widths[13] <= 100{
                wide+=widths[13];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[13];
            }
        }else if i == 'o'{
            if wide + widths[14] <= 100{
                wide+=widths[14];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[14];
            }
        }else if i == 'p'{
            if wide + widths[15] <= 100{
                wide+=widths[15];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[15];
            }
        }else if i == 'q'{
            if wide + widths[16] <= 100{
                wide+=widths[16];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[16];
            }
        }else if i == 'r'{
            if wide + widths[17] <= 100{
                wide+=widths[17];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[17];
            }
        }else if i == 's'{
            if wide + widths[18] <= 100{
                wide+=widths[18];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[18];
            }
        }else if i == 't'{
            if wide + widths[19] <= 100{
                wide+=widths[19];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[19];
            }
        }else if i == 'u'{
            if wide + widths[20] <= 100{
                wide+=widths[20];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[20];
            }
        }else if i == 'v'{
            if wide + widths[21] <= 100{
                wide+=widths[21];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[21];
            }
        }else if i == 'w'{
            if wide + widths[22] <= 100{
                wide+=widths[22];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[22];
            }
        }else if i == 'x'{
            if wide + widths[23] <= 100{
                wide+=widths[23];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[23];
            }
        }else if i == 'y'{
            if wide + widths[24] <= 100{
                wide+=widths[24];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[24];
            }
        }else if i == 'z'{
            if wide + widths[25] <= 100{
                wide+=widths[25];
            }else{
                lines+=1;
                wide = 0;
                wide+=widths[25];
            }
        }
    }

    solution.push(lines + 1);
    solution.push(wide);

    return solution;
}