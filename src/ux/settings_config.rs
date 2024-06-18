use crate::{generator::{GeneratorSettings, SourceSettings}, grids::GridSize};
use rand::Rng;

#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SettingsConfig {
    pub custom_override: bool,
    pub custom_settings: GeneratorSettings
}

struct Odds<T> {
    data: Vec<(T, u32)>
}

impl<T> Odds<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add(&mut self, entry: T, weight: u32) {
        let cumulative = self.data.last().map(|x| x.1).unwrap_or(0);
        self.data.push((entry, cumulative + weight));
    }

    pub fn get<R>(&mut self, rng: &mut R) -> Option<&T> where R: rand::Rng + ?Sized {
        let cumulative = self.data.last().map(|x| x.1)?;
        let cutoff = rng.next_u32() % cumulative;
        let index = self.data.partition_point(|entry| entry.1 < cutoff);
        Some(&self.data[index].0)
    }
}

#[derive(Debug, Clone, Copy)]
struct GridData {
    size: GridSize,
    missing: usize,
    swap_count: u8
}

fn get_random_grid_data<R>(rng: &mut R) -> GridData where R: rand::Rng + ?Sized {
    let mut size_odds = Odds::new();
    size_odds.add((GridSize::new(2, 3), 1), 3);
    size_odds.add((GridSize::new(1, 4), 0), 6);
    size_odds.add((GridSize::new(2, 4), 1), 9);
    size_odds.add((GridSize::new(3, 3), 2), 15);
    size_odds.add((GridSize::new(3, 4), 2), 8);
    size_odds.add((GridSize::new(4, 4), 3), 4);
    size_odds.add((GridSize::new(5, 5), 4), 2);
    let (size, missing) = *size_odds.get(rng).unwrap();
    let area = size.width * size.height;
    let par_swaps = match area {
        0..=5 => 2,
        6..=7 => 3,
        8..=9 => 4,
        10.. => 5
    };

    let mut swap_odds: Odds<u8> = Odds::new();
    swap_odds.add(par_swaps - 1, 10);
    swap_odds.add(par_swaps, 150);
    swap_odds.add(par_swaps + 1, 30);
    swap_odds.add(par_swaps + 2, 3);
    let swap_count = *swap_odds.get(rng).unwrap();

    GridData { size , missing, swap_count }
}

impl SettingsConfig {
    pub fn get_current_settings(&self) -> GeneratorSettings {
        if self.custom_override {
            self.custom_settings.clone()
        }
        else {
            self.get_random_settings()
        }
    }

    fn get_random_settings(&self) -> GeneratorSettings {
        let rng = &mut rand::thread_rng();

        let GridData { size, missing, swap_count } = get_random_grid_data(rng);

        GeneratorSettings {
            size,
            swap_count,
            missing_chance: 0.1,
            missing,

            stop_sources: SourceSettings::Maybe,
            rotator_sources: SourceSettings::Maybe,

            min_regions: 2,
            extra_region_chance: 0.2,

            extra_source_chance: rng.gen_range(0.075..0.2),

            intersection_chance: rng.gen_range(0.0..0.8),
            max_intersections: rng.gen_range(0..(size.width * size.height * 2 / 3)),

            knockout_loop_chance: 0.99 - rng.gen_range(0.0f32..0.5).powi(2),

            check_solution_len: (swap_count - 1).min(3) as usize,
            check_solution_retries: 3,
        }
    }
}