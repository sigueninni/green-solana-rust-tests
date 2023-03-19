// Import des dépendances Solana
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Structure de la liste de consommateurs
struct ConsumerList {
    pub consumers: Vec<Pubkey>,
}

// Structure de la liste de magasins
struct StoreList {
    pub stores: Vec<Pubkey>,
}

// Fonction qui initialise une nouvelle liste
fn new_list() -> Vec<Pubkey> {
    Vec::new()
}

// Fonction qui ajoute un élément à une liste
fn add_to_list(list: &mut Vec<Pubkey>, item: &Pubkey) {
    list.push(*item);
}

// Fonction qui supprime un élément d'une liste
fn remove_from_list(list: &mut Vec<Pubkey>, item: &Pubkey) {
    if let Some(index) = list.iter().position(|&x| x == *item) {
        list.remove(index);
    }
}

// Fonction qui retourne true si un élément est présent dans une liste
fn item_in_list(list: &Vec<Pubkey>, item: &Pubkey) -> bool {
    list.contains(item)
}

// Fonction d'entrée du programme
#[entrypoint]
fn process_program(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Récupération des comptes
    let accounts_iter = &mut accounts.iter();
    let consumer_list_account = next_account_info(accounts_iter)?;
    let store_list_account = next_account_info(accounts_iter)?;

    // Vérification que le programme est bien autorisé à modifier les comptes
    if consumer_list_account.owner != program_id || store_list_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Création de la liste de consommateurs
    let mut consumer_list = ConsumerList {
        consumers: new_list(),
    };

    // Création de la liste de magasins
    let mut store_list = StoreList {
        stores: new_list(),
    };

    // Décodage de l'instruction
    match instruction_data[0] {
        // Ajout d'un consommateur
        0 => {
            let consumer_account = next_account_info(accounts_iter)?;
            add_to_list(&mut consumer_list.consumers, &consumer_account.key);
        }

        // Suppression d'un consommateur
        1 => {
            let consumer_account = next_account_info(accounts_iter)?;
            remove_from_list(&mut consumer_list.consumers, &consumer_account.key);
        }

        // Vérification de l'existence d'un consommateur
        2 => {
            let consumer_account = next_account_info(accounts_iter)?;
            if item_in_list(&consumer_list.consumers, &consumer_account.key) {
                msg!("Consumer exists");
            } else {
                msg!("Consumer does not exist");
            }
        }

        // Ajout d'un magasin
        3 => {
            let store_account = next_account_info(accounts_iter)?;
            add_to_list(&mut store_list.stores, &store_account.key);
        }

        // Suppression d'un magasin
        4 => {
            let store_account = next_account_info(accounts_iter)?;
            remove_from_list(&mut store_list.stores,
