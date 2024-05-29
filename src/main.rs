use std::sync::Arc;
use std::time::Instant;
use rand::Rng;


mod hittable_list;
mod aabb;
mod hittable;
mod ray;
mod vec3;
mod utils;
mod material;
mod texture;
mod camera;
mod canvas;
mod colors;
mod sphere;
mod rect;


pub use hittable_list::*;
pub use aabb::*;
pub use hittable::*;
pub use ray::*;
pub use vec3::*;
pub use utils::*;
pub use material::*;
pub use texture::*;
pub use camera::*;
pub use canvas::*;
pub use colors::*;
pub use sphere::*;
pub use rect::*;


fn scene_driver(select: i32) -> HittableList {
    let mut objects = HittableList::new();
    
    //create matte colors and light source
    let light = Arc::new(DiffuseLight::from_color(Color::from(2.5, 2.5, 2.5)));
    //let light = Arc::new(DiffuseLight::from_color(Color::from(20., 20., 20.)));
    let red = Arc::new(Lambertian::from(Color::from(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from(Color::from(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from(Color::from(0.12, 0.45, 0.15)));
    let yellow = Arc::new(Lambertian::from(Color::from(0.65, 0.65, 0.05)));
    let blue = Arc::new(Lambertian::from(Color::from(0.05, 0.05, 0.65)));
    let skyblue = Arc::new(Lambertian::from(Color::from(0.53, 0.80, 0.92)));
    let brown = Arc::new(Lambertian::from(Color::from(0.47, 0.20, 0.08)));
    let darkbrown = Arc::new(Lambertian::from(Color::from(0.345, 0.17, 0.08)));
    
    
    if select == 0 {

        objects.add(Arc::new(Sphere::new(
            Point3::from(4., -0.5, 0.),
            1.,
            white.clone(),
        )));
        objects.add(Arc::new(Sphere::new(
            Point3::from(0., 2., 0.),
            2.,
            green.clone(),
        )));
        objects.add(Arc::new(Sphere::new(
            Point3::from(-3., 1., -1.5),
            1.,
            white.clone(),
        )));
        objects.add(Arc::new(Sphere::new(
            Point3::from(3., -1., -1.2),
            1.2,
            yellow.clone(),
        )));

        let difflight = Arc::new(DiffuseLight::from_color(Color::from(7., 7., 7.)));
        objects.add(Arc::new(Sphere::new(
            Point3::from(-2., -1., -3.),
            1.2,
            difflight.clone()
        )));
            
        objects.add(Arc::new(YzRect::from(-1.5, -0.5, 1., 4., -5., light.clone())));
        objects.add(Arc::new(YzRect::from(0., 10., 0., 10., 0., red.clone())));
        objects.add(Arc::new(XyRect::from(-10., 4., -2.5, 2.5, -3.75, blue.clone())));

    }
    else if select == 1 {
        let brightlight = Arc::new(DiffuseLight::from_color(Color::from(20., 20., 20.)));
        objects.add(Arc::new(YzRect::from(-5., 5., -5., 5., -16., brightlight.clone())));

        objects.add(Arc::new(YzRect::from(0., 10., -10., 10., 0., skyblue.clone())));
        objects.add(Arc::new(YzRect::from(-10., 0., -10., 10., -1., brown.clone())));
        
        objects.add(Arc::new(YzRect::from(-0.5, 1.5, -0.3, 0.3, -5., darkbrown.clone())));
        objects.add(Arc::new(Sphere::new(Point3::from(-5., 1.8, 0.), 0.6, green.clone())));
        objects.add(Arc::new(Sphere::new(Point3::from(1., 0., -4.), 2., yellow.clone())));
    }
    else if select == 2 {
        objects.add(Arc::new(YzRect::from(-10., 10., -10., 10., -15., light.clone())));
        
        let mut randlist = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            randlist.push(Arc::new(Lambertian::from(Color::from(rng.gen_range(0.10..0.95), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)))));
        }
        
        objects.add(Arc::new(Quadrilateral::from((-0.8, -0.4), (-0.2, 0.4), (0.4, 0.6), (0.2, -0.2), -8., red.clone())));
        objects.add(Arc::new(Quadrilateral::from((-0.8, -0.4), (-0.2, 0.4), (-0.6, 0.3), (-0.9, -0.2), -8., blue.clone())));
        objects.add(Arc::new(Quadrilateral::from((-1.2, -0.3), (-1.1, 0.3), (-0.6, 0.3), (-0.9, -0.2), -8., green.clone())));
        objects.add(Arc::new(Quadrilateral::from((-0.8, -0.4), (-0.9, -0.2), (-1.2, -0.3), (-1.5, -0.9), -8., brown.clone())));
        objects.add(Arc::new(Quadrilateral::from((-0.8, -0.4), (-1.5, -0.9), (-0.8, -1.25), (-0.6, -0.6), -8., skyblue.clone())));
        objects.add(Arc::new(Quadrilateral::from((-0.8, -0.4), (-0.6, -0.6), (0., -0.4), (0.2, -0.2), -8., yellow.clone())));
        objects.add(Arc::new(Quadrilateral::from((-0.6, -0.6), (-0.8, -1.25), (-0.1, -0.6), (0., -0.4), -8., randlist[0].clone())));
        objects.add(Arc::new(Quadrilateral::from((0., -0.4), (-0.1, -0.6), (0.7, -1.0), (0.2, -0.2), -8., randlist[1].clone())));
        objects.add(Arc::new(Quadrilateral::from((0.4, 0.6), (0.8, 0.7), (0.6, 0.), (0.2, -0.2), -8., randlist[2].clone())));
        objects.add(Arc::new(Quadrilateral::from((0.2, -0.2), (0.6, 0.), (1.2, -0.3), (0.7, -1.0), -8., randlist[3].clone())));
        objects.add(Arc::new(Quadrilateral::from((1.2, 0.3), (1.4, 0.8), (0.8, 0.7), (0.6, 0.),  -8., randlist[4].clone())));
        objects.add(Arc::new(Quadrilateral::from((-0.1, -0.6), (0.7, -1.0), (0.4, -1.25), (-0.8, -1.25),  -8., randlist[5].clone())));
        objects.add(Arc::new(Quadrilateral::from((1.2, -0.3), (0.6, 0.), (1.2, 0.3), (1.6, 0.4),  -8., randlist[6].clone())));
        objects.add(Arc::new(Quadrilateral::from((1.2, 0.3), (1.4, 0.8), (1.5, 1.25), (1.6, 0.4),  -8., randlist[7].clone()))); //top point

        //Sorted after this point; order:(top left, top right, botom right, bottom left)
        objects.add(Arc::new(Quadrilateral::from((1.5, 1.25), (1.8, 1.25), (2.0, 0.5), (1.6, 0.4),  -8., randlist[8].clone())));
        objects.add(Arc::new(Quadrilateral::from((1.8, 1.25), (2.25, 1.25), (2.25, 0.4), (2.0, 0.5),  -8., randlist[9].clone()))); 
        objects.add(Arc::new(Quadrilateral::from((1.6, 0.4), (2.0, 0.5), (2.25, 0.4), (2.25, 0.1),  -8., randlist[10].clone()))); 
        objects.add(Arc::new(Quadrilateral::from((1.6, 0.4), (2.25, 0.1), (2.25, -0.6), (1.2, -0.3),  -8., randlist[11].clone()))); 
        objects.add(Arc::new(Quadrilateral::from((1.2, -0.3), (2.25, -0.6), (1.8, -0.8), (0.7, -1.0),  -8., randlist[12].clone()))); 
        objects.add(Arc::new(Quadrilateral::from((1.8, -0.8), (2.25, -0.6), (2.25, -1.25), (1.6, -1.1),  -8., randlist[13].clone()))); 
        objects.add(Arc::new(Quadrilateral::from((0.7, -1.0), (1.8, -0.8), (1.6, -1.1), (0.4, -1.25),  -8., randlist[14].clone()))); 
        objects.add(Arc::new(Quadrilateral::from((1.6, -1.1), (1.6, -1.1), (2.25, -1.25), (0.4, -1.25),  -8., randlist[15].clone()))); //end of the bottom closure

        objects.add(Arc::new(Quadrilateral::from((1.3, 1.25), (1.5, 1.25), (1.4, 0.8), (0.8, 0.7),  -8., randlist[16].clone()))); 
        objects.add(Arc::new(Quadrilateral::from((0.9, 1.1), (1.3, 1.25), (0.8, 0.7), (0.4, 0.6), -8., randlist[17].clone()))); 
        objects.add(Arc::new(Quadrilateral::from((0.7, 1.25), (1.3, 1.25), (0.9, 1.1), (0.3, 0.8), -8., randlist[18].clone()))); 
        objects.add(Arc::new(Quadrilateral::from((0.3, 0.8), (1.3, 1.25), (0.4, 0.6), (-0.2, 0.4), -8., randlist[19].clone()))); 
        objects.add(Arc::new(Quadrilateral::from((0.1, 1.25), (0.7, 1.25), (0.3, 0.8),(-0.5, 0.9), -8., randlist[20].clone()))); 
        objects.add(Arc::new(Quadrilateral::from((-0.5, 0.9), (0.3, 0.8), (-0.2, 0.4), (-0.6, 0.3), -8., randlist[21].clone())));
        objects.add(Arc::new(Quadrilateral::from((-0.4, 1.25), (0.1, 1.25), (-0.5, 0.9), (-0.8, 0.8), -8., randlist[22].clone()))); 
        objects.add(Arc::new(Quadrilateral::from((-0.9, 1.25), (-0.4, 1.25), (-0.8, 0.8), (-1.1, 0.9), -8., randlist[23].clone()))); 
        objects.add(Arc::new(Quadrilateral::from((-1.1, 0.9), (-0.8, 0.8), (-0.8, 0.8), (-1.1, 0.3), -8., randlist[99].clone()))); //triangle repair
        objects.add(Arc::new(Quadrilateral::from((-0.8, 0.8), (-0.5, 0.9), (-0.6, 0.3), (-1.1, 0.3), -8., randlist[24].clone())));
        objects.add(Arc::new(Quadrilateral::from((-1.9, 0.9), (-1.1, 0.9), (-1.1, 0.3), (-1.7, 0.5), -8., randlist[25].clone())));
        objects.add(Arc::new(Quadrilateral::from((-1.5, 1.25), (-0.9, 1.25), (-1.1, 0.9), (-1.9, 0.9), -8., randlist[26].clone())));
        objects.add(Arc::new(Quadrilateral::from((-2.25, 1.25), (-1.5, 1.25), (-1.9, 0.9), (-2.25, 0.8), -8., randlist[27].clone()))); //top left
        objects.add(Arc::new(Quadrilateral::from((-2.25, 0.8), (-1.9, 0.9), (-1.7, 0.5), (-2.25, 0.1), -8., randlist[28].clone())));
        objects.add(Arc::new(Quadrilateral::from((-2.25, 0.1), (-1.7, 0.5), (-1.1, 0.3), (-1.9, -0.1), -8., randlist[29].clone())));
        objects.add(Arc::new(Quadrilateral::from((-1.9, -0.1), (-1.1, 0.3), (-1.2, -0.3), (-1.8, -0.4), -8., randlist[30].clone())));
        objects.add(Arc::new(Quadrilateral::from((-1.8, -0.4), (-1.2, -0.3), (-1.5, -0.9), (-2.0, -0.8), -8., randlist[31].clone())));
        objects.add(Arc::new(Quadrilateral::from((-2.0, -0.8), (-1.5, -0.9), (-0.8, -1.25), (-1.7, -1.25), -8., randlist[32].clone())));
        objects.add(Arc::new(Quadrilateral::from((-2.25, -0.7), (-2.0, -0.8), (-1.7, -1.25), (-2.25, -1.25), -8., randlist[33].clone()))); //bottom left
        objects.add(Arc::new(Quadrilateral::from((-2.25, -0.4), (-1.8, -0.4), (-2.0, -0.8), (-2.25, -0.7), -8., randlist[34].clone())));
        objects.add(Arc::new(Quadrilateral::from((-2.25, 0.1), (-1.9, -0.1), (-1.8, -0.4), (-2.25, -0.4), -8., randlist[35].clone())));
        

    }


        objects
}


fn format_bar(bar: &mut String , percentage: f32){
    bar.clear();
    let to_color = (percentage / 2.).floor() as i32;
    bar.push('[');
    bar.extend((0..to_color).map(|_| '#'));
    bar.extend((0..(50 - to_color)).map(|_| '.'));
    bar.push(']');
}



fn main() {
    let time = Instant::now(); // Time counter
    //Image info
    let image_width : usize = 800;
    let aspect_ratio : f32 = 16.0/9.0;

    let samples_per_pixel : usize = 5000;
    const MAX_DEPTH : usize = 50;

    let mut world = HittableList::new();

    //Camera
    let lookfrom = Point3::from(-15., 0., 0.);
    let lookat = Point3::from(0., 0., 0.);
    let vup = Vec3::from(0., 1., 0.);
    let dist_to_focus: f32 = 5.0;
    let aperture: f32 = 0.0;
    let vfov = 20.;
    let background = Color::new();

    let image_height: usize = (image_width as f32 / aspect_ratio) as usize;

    world = scene_driver(0);

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.,
        1.,
    );

    let render_pixel = |i, j| -> Color {
        let mut pixel_color = Color::new();
        for _ in 0..samples_per_pixel {
            let u = (i as f32 + random_double(0., 1.)) / ((image_width - 1) as f32);
            let v = (j as f32 + random_double(0., 1.)) / ((image_height - 1) as f32);
            let r = cam.get_ray(u, v);
            pixel_color.add(ray_color(r, &background, &world, MAX_DEPTH as i32));
        }
        pixel_color
    };

    // Render
    // let c = Canvas::from_fn(
    //     image_width as usize,
    //     image_height as usize,
    //     samples_per_pixel as usize,
    //     render_pixel,
    // );

    let c: Canvas = Canvas::from_fn_parallel(
        image_width as usize,
        image_height as usize,
        samples_per_pixel as usize,
        render_pixel,
    );

    // let mut bar = String::with_capacity(52);
    // let c = Canvas::from_fn_parallel_with_progress(
    //     image_width as usize,
    //     image_height as usize,
    //     samples_per_pixel as usize,
    //     render_pixel,
    //     move |total, num_done| {
    //         let percentage = ((num_done as f32 / total as f32) * 100.).min(100.);
    //         format_bar(&mut bar , percentage);
    //         eprint!(
    //             "\r{:?} {:.2}%",
    //             bar.as_str(),
    //             ((num_done as f32 / total as f32) * 100.).min(100.)
    //         );
    //     },
    // );
    
    c.write_header();
    c.write_pixels();
    let elapsed = time.elapsed();
    eprint!("\nDone in {:.2}s\n", elapsed.as_secs_f32());

}
