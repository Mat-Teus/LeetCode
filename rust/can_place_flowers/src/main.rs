fn main() {
    let vec = vec![0,0];
    let n = 2;

    println!("{}", can_place_flowers(vec, n));
}

pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut n = n;
    let mut flowerbed = flowerbed;

    if flowerbed.len() == 1 && flowerbed[0] == 0{
        return true;
    }

    if flowerbed.len() >= 2 && flowerbed[0] == 0 && flowerbed[1] == 0{
        flowerbed[0] = 1;
        n-=1;
    }

    for i in 1..flowerbed.len() - 1{
        if flowerbed[i - 1] != 1 && flowerbed[i + 1] != 1 && flowerbed[i] != 1{
            flowerbed[i] = 1;
            n-=1;
        }
    }

    if flowerbed.len() >= 2 && flowerbed[flowerbed.len() - 1] == 0 && flowerbed[flowerbed.len() - 2] != 1{
        n-=1;
    }

    if n <= 0{
        return true;
    }else{
        return false;
    }
}