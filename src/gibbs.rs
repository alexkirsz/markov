use rand::prelude::*;

enum CliqueType {
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

const NUM_CLASSES: usize = 8;

pub struct MK {
    width: usize,
    height: usize,

    dist: Vec<[usize; NUM_CLASSES]>,
    x: Vec<usize>,
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
                        let u: Vec<_> = (0..NUM_CLASSES)
                            .map(|c| self.energie_cliques(&out, (x, y), c, clique_type, beta))
                            .collect();

                        let pr: Vec<_> = u.iter().map(|x| (-x / t).exp()).collect();

                        /* the probability distribution */
                        let pr_tot = pr.sum();
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
    ) -> (
        f32, /* energie du pixel */
        f32, /* energie pouvant être attrib au pixel */
    ) {
        let c_dist = self.dist[self.pos(x, y)][c];

        let mut err = 0;
        match clique_type {
            Conn4 => {
                /* energie des cliques d'ordre 1 verticale */
                if (x + 1 <= self.width) && (c != self.get(x + 1, y)) {
                    err += 1;
                }
                if (x - 1 >= 1) && (c != self.get(x - 1, y)) {
                    err += 1;
                }
                /* energie des cliques d'ordre 1 horizontale */
                if (y + 1 <= self.height) && (c != self.get(x, y + 1)) {
                    err += 1;
                }
                if (y - 1 >= 1) && (c != self.get(x, y - 1)) {
                    err += 1;
                }
            }
            Conn8 => {
                /* energie des cliques d'ordre 1 verticale */
                if (x + 1 <= self.width) && (c != self.get(x + 1, y)) {
                    err += 1;
                }
                if (x - 1 >= 1) && (c != self.get(x - 1, y)) {
                    err += 1;
                }
                /* energie des cliques d'ordre 1 horizontale */
                if (y + 1 <= self.height) && (c != self.get(x, y + 1)) {
                    err += 1;
                }
                if (y - 1 >= 1) && (c != self.get(x, y - 1)) {
                    err += 1;
                }
                /* energie des cliques d'ordre 1 diagonale */
                if (x + 1 <= self.width) && (y + 1 <= self.height) && (c != self.get(x + 1, y + 1))
                {
                    err += 1;
                }
                if (x + 1 <= self.width) && (y - 1 >= 1) && (c != self.get(x + 1, y - 1)) {
                    err += 1;
                }
                /* energie des cliques d'ordre 1 diagonale */
                if (x - 1 >= 1) && (y + 1 <= self.height) && (c != self.get(x - 1, y + 1)) {
                    err += 1;
                }
                if (x - 1 >= 1) && (y - 1 >= 1) && (c != self.get(x - 1, y - 1)) {
                    err += 1;
                }
            }
        }
        (c_dist + beta * err, c_dist + beta * 0.5 * err)
    }

    fn recuit_simule(
        n: usize,
        clique_type: CliqueType,
        beta: f32,
        t_init: f32,
    ) -> (Vec<[f32; NUM_CLASSES]>, Vec<f32>) {
        for k in 0..n {
            println!("iteration {}/{}", k, n);
            let t = t_init * (0.1f32.powf(k / n) - 0.05);
            gibbs_sampler(1, t, clique_type, beta);
        }
    }
}
