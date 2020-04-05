use ror2_command::average_distance2;

fn main() {
    let green_items = image::open("./screens/green_items.jpg")
        .unwrap();
    let green_items = green_items.as_rgb8()
        .unwrap();

    let red_items = image::open("./screens/red_items.jpg")
        .unwrap();
    let red_items = red_items.as_rgb8()
        .unwrap();

    let white_items = image::open("./screens/white_items.jpg")
        .unwrap();
    let white_items = white_items.as_rgb8()
        .unwrap();

    let _green = (118, 237, 34);
    let _red = (212, 83, 54);
    let _white = (242, 246, 232);

    let colors = [
        ("green", _green),
        ("red", _red),
        ("white", _white),
    ];

    // these parameters work very well on the test images!
    let span = 4;
    let y = 1080 / 2;
    let x = 672;

    println!("Computing average distance for each image");

    for (name, value) in &colors {
        println!("Avg distance for {}", name);

        let mut dist;
        dist = average_distance2(green_items, value, x, y, span);
        println!("\tgreen items: {}", dist);

        dist = average_distance2(red_items, value, x, y, span);
        println!("\tred items: {}", dist);

        dist = average_distance2(white_items, value, x, y, span);
        println!("\twhite items: {}", dist);
    }
}
