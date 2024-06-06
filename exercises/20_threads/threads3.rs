// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q); // Crée une référence atomique partagée au vecteur q
    let qc1 = Arc::clone(&qc); // Clone de la référence atomique pour le premier thread
    let qc2 = Arc::clone(&qc); // Clone de la référence atomique pour le deuxième thread
    let tx1 = tx.clone(); // Clone de la chaîne d'envoi pour le premier thread

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap(); // Envoie une copie de la valeur à travers le canal
            thread::sleep(Duration::from_secs(1)); // Attente de 1 seconde
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx.send(*val).unwrap(); // Envoie une copie de la valeur à travers le canal
            thread::sleep(Duration::from_secs(1)); // Attente de 1 seconde
        }
    });
}

#[test]
fn main() {
    let (tx, rx) = mpsc::channel(); // Crée un canal de communication entre les threads
    let queue = Queue::new(); // Crée une nouvelle file d'attente
    let queue_length = queue.length; // Récupère la longueur de la file d'attente

    send_tx(queue, tx); // Appelle la fonction d'envoi pour démarrer les threads d'envoi

    let mut total_received: u32 = 0; // Initialise le total des valeurs reçues à 0
    for received in rx { // Boucle sur les valeurs reçues à travers le canal de réception
        println!("Got: {}", received); // Affiche la valeur reçue
        total_received += 1; // Incrémente le total des valeurs reçues
    }

    println!("total numbers received: {}", total_received); // Affiche le total des valeurs reçues
    assert_eq!(total_received, queue_length) // Vérifie si le total des valeurs reçues correspond à la longueur de la file d'attente
}
