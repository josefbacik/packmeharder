use anyhow::Result;
use clap::Parser;
use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Parser)]
struct PackMeHarder {
    #[arg(long)]
    csv: String,
    #[arg(long)]
    suitcase_weight: f64,
    #[arg(long)]
    max_weight: f64,
}

#[derive(Debug, Deserialize)]
struct Medicine {
    medication: String,
    quantity: u64,
    weight: f64,
}

#[derive(Debug)]
struct Suitcase {
    weight: f64,
    medicines: Vec<Medicine>,
}

fn main() -> Result<()> {
    let opts = PackMeHarder::parse();
    let mut reader = Reader::from_path(opts.csv).unwrap();
    let medicine: Vec<Medicine> = reader.deserialize().collect::<Result<Vec<Medicine>, _>>()?;
    let mut suitcases = Vec::new();
    suitcases.push(Suitcase {
        weight: opts.suitcase_weight,
        medicines: Vec::new(),
    });

    for med in medicine.iter() {
        for _ in 0..med.quantity {
            let mut index = suitcases.len();
            let mut min_weight = 0 as f64;

            for (i, suitcase) in suitcases.iter().enumerate() {
                let weight = suitcase.weight + med.weight;
                if weight <= opts.max_weight && (index == suitcases.len() || weight < min_weight) {
                    index = i;
                    min_weight = weight;
                }
            }

            if index == suitcases.len() {
                suitcases.push(Suitcase {
                    weight: opts.suitcase_weight,
                    medicines: Vec::new(),
                });
                index = suitcases.len() - 1;
            }
            let suitcase = suitcases.get_mut(index).unwrap();
            suitcase.weight += med.weight;
            if let Some(med_index) = suitcase
                .medicines
                .iter()
                .position(|m| m.medication == med.medication)
            {
                suitcase.medicines[med_index].quantity += 1;
                suitcase.medicines[med_index].weight += med.weight;
            } else {
                suitcase.medicines.push(Medicine {
                    medication: med.medication.clone(),
                    quantity: 1,
                    weight: med.weight,
                });
            }
        }
    }

    for (i, suitcase) in suitcases.iter().enumerate() {
        println!("Suitcase {}", i + 1);
        for med in suitcase.medicines.iter() {
            println!("{} x {}", med.quantity, med.medication);
        }
        println!("Total weight: {}", suitcase.weight);
        println!();
    }

    Ok(())
}
