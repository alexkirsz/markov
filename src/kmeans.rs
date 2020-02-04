use super::vec3::Vec3;
use rand::prelude::*;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Point {
    pub coords: Vec3,
    pub cluster: usize,
    pub distance_to_cluster: f32,
}

fn compute_cluster_barycenters(clustered_points: &[Point], n_clusters: usize) -> Vec<Vec3> {
    (0..n_clusters)
        .into_par_iter()
        .map(|cluster| {
            let cluster_points: Vec<_> = clustered_points
                .iter()
                .filter_map(|pt| {
                    if pt.cluster == cluster {
                        Some(pt)
                    } else {
                        None
                    }
                })
                .collect();
            let len = cluster_points.len() as f32;
            cluster_points
                .into_par_iter()
                .map(|pt| pt.coords)
                .sum::<Vec3>()
                / len
        })
        .collect()
}

fn assign_to_barycenters(clustered_points: &mut [Point], barycenters: &[Vec3]) {
    for (cluster, barycenter) in barycenters.iter().enumerate() {
        for point in clustered_points.iter_mut() {
            let dist = (point.coords - barycenter).norm();
            if dist < point.distance_to_cluster {
                point.cluster = cluster;
                point.distance_to_cluster = dist;
            }
        }
    }
}

fn min_max(elements: &[f32]) -> (f32, f32) {
    use std::cmp::Ordering;

    (
        *elements
            .iter()
            .min_by(|a, b| {
                if a < b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap(),
        *elements
            .iter()
            .max_by(|a, b| {
                if a < b {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap(),
    )
}

pub fn random_barycenters(coords: &[Vec3], n_clusters: usize) -> Vec<Vec3> {
    let (min_x, max_x) = min_max(&coords.iter().map(|v| v.x).collect::<Vec<_>>());
    let (min_y, max_y) = min_max(&coords.iter().map(|v| v.y).collect::<Vec<_>>());
    let (min_z, max_z) = min_max(&coords.iter().map(|v| v.z).collect::<Vec<_>>());
    let x_range = max_x - min_x;
    let y_range = max_y - min_y;
    let z_range = max_z - min_z;

    let mut rng = rand::thread_rng();

    (0..n_clusters)
        .map(|_| {
            Vec3::new(
                min_x + rng.gen::<f32>() * x_range,
                min_y + rng.gen::<f32>() * y_range,
                min_z + rng.gen::<f32>() * z_range,
            )
        })
        .collect()
}

pub fn kmeans(
    coords: &[Vec3],
    initial_barycenters: &[Vec3],
    stop_dist: f32,
) -> (Vec<Point>, Vec<Vec3>) {
    let n_clusters = initial_barycenters.len();

    assert!(n_clusters > 0);
    assert!(coords.len() > 0);

    let mut rng = rand::thread_rng();

    let mut cluster_assignments: Vec<_> = (0..coords.len()).map(|i| i % n_clusters).collect();
    cluster_assignments.shuffle(&mut rng);

    let mut clustered_points: Vec<_> = coords
        .iter()
        .zip(cluster_assignments)
        .map(|(coords, cluster)| Point {
            coords: *coords,
            cluster,
            distance_to_cluster: std::f32::INFINITY,
        })
        .collect();

    let mut barycenters = initial_barycenters.to_owned();

    assign_to_barycenters(&mut clustered_points, &barycenters);

    let mut dist = std::f32::INFINITY;
    while dist > stop_dist {
        let next_barycenters = compute_cluster_barycenters(&clustered_points, n_clusters);

        dist = next_barycenters
            .iter()
            .zip(&barycenters)
            .map(|(a, b)| (a - b).norm())
            .sum();

        barycenters = next_barycenters;

        assign_to_barycenters(&mut clustered_points, &barycenters);
    }

    (clustered_points, barycenters)
}
