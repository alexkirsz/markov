use super::vec3::Vec3;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Point {
    pub coords: Vec3,
    pub cluster: usize,
    pub distance_to_cluster: f32,
}

fn compute_cluster_barycenters(barycenters: &mut [Vec3], clustered_points: &[Point]) {
    for (cluster, barycenter) in barycenters.iter_mut().enumerate() {
        *barycenter = {
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
                .into_iter()
                .fold(Vec3::zero(), |acc, pt| acc + pt.coords)
                / len
        };
    }
}

pub fn kmeans(coords: &[Vec3], n_classes: usize) -> Vec<Point> {
    assert!(n_classes > 0);
    assert!(coords.len() > 0);

    let mut rng = rand::thread_rng();

    let mut cluster_assignments: Vec<_> = (0..coords.len()).map(|i| i % n_classes).collect();
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
    let mut cluster_barycenters: Vec<_> = (0..n_classes).map(|_| Vec3::zero()).collect();

    let mut changed = true;
    while changed {
        changed = false;

        compute_cluster_barycenters(&mut cluster_barycenters, &clustered_points);

        for (cluster, barycenter) in cluster_barycenters.iter().enumerate() {
            for point in &mut clustered_points {
                let dist = (point.coords - barycenter).norm();
                if dist < point.distance_to_cluster {
                    point.cluster = cluster;
                    point.distance_to_cluster = dist;
                    changed = true;
                }
            }
        }
    }

    clustered_points
}
