use crate::hittable;

fn create_random_scene() -> hittable::HittableList {
    let mut rng = rand::thread_rng();
    let mut world = hittable::HittableList::new();

    let ground_material = Material::Lambertian(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..12 {
        for b in -11..12 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + rng.gen::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
                    let material = Material::Lambertian(Lambertian::new(albedo));
                    world.push(Sphere::new(center, 0.2, material));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = rng.gen::<f64>();
                    let material = Material::Metal(Metal::new(albedo, fuzz));
                    world.push(Sphere::new(center, 0.2, material));
                } else {
                    let material = Material::Dielectric(Dielectric::new(1.5));
                    world.push(Sphere::new(center, 0.2, material));
                }
            }
        }
    }

    // fixed objects
    let material1 = Material::Dielectric(Dielectric::new(1.5));
    world.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Material::Lambertian(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    world
}
