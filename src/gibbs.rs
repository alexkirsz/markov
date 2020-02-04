// Originally from http://www.tsi.enst.fr/pages/enseignement/ressources/beti/markov/code.txt
// Adapted to Rust by Victor Collod and Alexandre Kirszenberg

use rand::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum CliqueType {
    Conn4,
    Conn8,
}

fn sample_dist(dist: Vec<f32>) -> usize {
    let r: f32 = rand::random();
    let mut cum_prob = 0.0f32;
    for (i, cur_prob) in dist.iter().enumerate() {
        cum_prob += cur_prob;
        if cum_prob >= r {
            return i;
        }
    }
    dist.len()
}

pub struct MK {
    pub width: usize,
    pub height: usize,

    pub num_classes: usize,
    pub dist: Vec<Vec<f32>>,
    pub x: Vec<usize>,
}

impl MK {
    fn pos(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn get(&self, x: usize, y: usize) -> usize {
        self.x[self.pos(x, y)]
    }

    fn gibbs_sampler(
        &self,
        n: usize,                /* nombre d'itérations */
        t: f32,                  /* température */
        clique_type: CliqueType, /* type de clique */
        beta: f32,               /* coeff de régularisation de potts*/
    ) -> Vec<usize> /* réalisation du champ de markov */ {
        let mut out = vec![0usize; self.width * self.height];

        for _ in 0..n {
            for (x_start, y_start) in vec![(0, 0), (1, 1), (0, 1), (1, 0)] {
                for x in (x_start..self.width).step_by(2) {
                    for y in (y_start..self.height).step_by(2) {
                        /* for each class, find the energy for the current pixel */
                        let u: Vec<_> = (0..self.num_classes)
                            .map(|c| self.energie_cliques(&out, (x, y), c, clique_type, beta))
                            .collect();

                        let pr: Vec<_> = u.iter().map(|x| (-x / t).exp()).collect();

                        /* the probability distribution */
                        let pr_tot = pr.iter().sum::<f32>();
                        let p: Vec<_> = pr.iter().map(|x| x / pr_tot).collect();

                        /* take a random sample following the given distribution */
                        out[self.pos(x, y)] = sample_dist(p);
                    }
                }
            }
        }

        out
    }

    fn energie_cliques(
        &self,
        labels: &[usize],
        (x, y): (usize, usize),
        c: usize,
        clique_type: CliqueType,
        beta: f32,
    ) -> f32 {
        let c_dist = self.dist[self.pos(x, y)][c];

        let mut err = 0;
        match clique_type {
            Conn4 => {
                /* energie des cliques d'ordre 1 verticale */
                if x != self.width - 1 && (c != self.get(x + 1, y)) {
                    err += 1;
                }
                if x != 0 && (c != self.get(x - 1, y)) {
                    err += 1;
                }
                /* energie des cliques d'ordre 1 horizontale */
                if y != self.height - 1 && (c != self.get(x, y + 1)) {
                    err += 1;
                }
                if y != 0 && (c != self.get(x, y - 1)) {
                    err += 1;
                }
            }
            Conn8 => {
                /* energie des cliques d'ordre 1 verticale */
                if x != self.width - 1 && (c != self.get(x + 1, y)) {
                    err += 1;
                }
                if x != 0 && (c != self.get(x - 1, y)) {
                    err += 1;
                }
                /* energie des cliques d'ordre 1 horizontale */
                if y != self.height - 1 && (c != self.get(x, y + 1)) {
                    err += 1;
                }
                if y != 0 && (c != self.get(x, y - 1)) {
                    err += 1;
                }
                /* energie des cliques d'ordre 1 diagonale */
                if x != self.width - 1 && y != self.height - 1 && (c != self.get(x + 1, y + 1)) {
                    err += 1;
                }
                if x != self.width - 1 && y != 0 && (c != self.get(x + 1, y - 1)) {
                    err += 1;
                }
                /* energie des cliques d'ordre 1 diagonale */
                if x != 0 && y != self.height - 1 && (c != self.get(x - 1, y + 1)) {
                    err += 1;
                }
                if x != 0 && y != 0 && (c != self.get(x - 1, y - 1)) {
                    err += 1;
                }
            }
        }
        c_dist + beta * err as f32
    }

    pub fn recuit_simule(
        mut self,
        n: usize,
        clique_type: CliqueType,
        beta: f32,
        t_init: f32,
    ) -> Vec<usize> {
        for k in 0..n {
            println!("iteration {}/{}", k, n);
            let t = t_init * (0.1f32.powf(k as f32 / n as f32) - 0.05);
            self.x = self.gibbs_sampler(1, t, clique_type, beta);
        }

        self.x
    }
}
