fn main() {
    let s = String::from("LL");

    println!("{}", judge_circle(s));
}

pub fn judge_circle(moves: String) -> bool {
    let mut count_d = 0;
    let mut count_r = 0;

    for i in moves.chars(){
        if i == 'D'{
            count_d+= 1;
        }else if i == 'U'{
            count_d -= 1;
        }else if i == 'R'{
            count_r += 1;
        }else if i == 'L'{
            count_r -= 1;
        }
    }

    if count_d == 0 && count_r == 0{
        return true;
    }else{
        return false;
    }
}