pub fn roman_to_int(s: String) -> i32 {
    let mut map = [0; 26];
    map[8] = 1;
    map[21] = 5;
    map[23] = 10;
    map[11] = 50;
    map[2] = 100;
    map[3] = 500;
    map[12] = 1000;

    let (mut sum, _n) = (0, s.len() - 1);
    let mut s = s.into_bytes().into_iter().map(|b| (b - 65) as usize);
    let mut cur = s.next().unwrap();
    while let Some(next) = s.next() {
        sum += if map[cur] >= map[next] { map[cur] } else { -map[cur] };
        cur = next;
    }
    sum + map[cur]
}