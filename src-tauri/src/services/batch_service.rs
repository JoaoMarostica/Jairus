use crate::{ models::batch::Batch, repositories::batch_repository::BatchRepository};
use crate::models::stats::*; 
use std::collections::HashMap;
use serde::Serialize; 
use std::{fs, io::BufWriter, path::{Path, PathBuf}};
use anyhow::{Result, anyhow};

use genpdf::{
    elements::{StyledElement, Paragraph, TableLayout},
    fonts::{FontFamily, FontData, from_files}, 
    style::{Style, StyledString},
    Alignment, Document, Margins
};

#[derive(Serialize, Debug)]
pub struct StockReportEntry {
    pub seed_name: String,
    pub total_weight_in_stock: i32,
    pub total_sacks_in_stock: i32,
}

#[derive(Serialize, Debug, Clone)] 
pub struct DetailedBatchReportEntry {
    pub batch_number: i32,
    pub creation_year: i32,
    pub cultivar: String, 
    pub treatment: String, 
    pub brand: String,
    pub sack_weight: i32,
    pub sack_amount: i32,
    pub total_weight: i32,
    pub status: i32,
    pub origin: Option<String>,
}

pub struct BatchService {
    repo: BatchRepository
}

impl BatchService {
    pub fn new() -> Self {
        Self { repo: BatchRepository::new() }
    }
    
    pub fn create(&mut self, batch: &Batch) -> Result<Batch,String> {
        if batch.batch_month < 0 || batch.batch_month > 11 {
            return Err("Month beyond valid range".to_string())
        } else if batch.sack_amount <= 0 || batch.sack_weight <= 0 || batch.pureness_score <= 0.0 {
            return Err("No input can be 0 or negative".to_string())
        } else if batch.total_weight > 10000 || batch.total_weight < batch.sack_weight {
            return Err(format!("Total weight cannot be above 10,000 or below {}", batch.sack_weight))
        } else if batch.total_pureness_score <= batch.pureness_score {
            return Err(format!("Total pureness cannot be below {}", batch.pureness_score))
        } else if batch.batch_status != 1 {
            return Err("On creation batches must be active".to_string())
        } else if batch.deleted_at != None {
            return Err("On creation batches cannot have been deleted".to_string())
        }
        
        match self.repo.insert(batch) {
            Ok(result) => Ok(result),
            Err(e) => Err(e.to_string())
        }
    }
    
    pub fn read_id(&mut self, id: &(i32, i32)) -> Result<Batch,String> {
        match self.repo.select_id(&id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No batch with id {}/{} found", id.0, id.1)),
            Err(e) => Err(e.to_string())
        }
    }

    pub fn read_year(&mut self, year:i32) -> Result<Vec<Batch>, String>{
        match self.repo.select_year(year) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No batches in year {}", year)),
            Err(e) => Err(e.to_string())
        }
    }
    
    pub fn read_all(&mut self) -> Result<Vec<Batch>,String> {
        match self.repo.select_all() {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err("No batches in the database".to_string()),
            Err(e) => Err(e.to_string())
        }
    }
 
    pub fn read_statistics(&mut self) -> Result<BatchStatistics, String> {
        if let Ok(Some(all_batches)) = self.repo.select_all() {

            let total_batches = all_batches.len();
            
            let total_weight:i32 = all_batches.iter()
                .map(|batch| batch.total_weight)
                .sum();

            let mut years_map: HashMap<i32,(usize, i32)> = HashMap::new();
            for batch in &all_batches {
                let entry =
                years_map.entry(batch.batch_year).or_insert((0, 0));
                entry.0 += 1;
                entry.1 += batch.total_weight;
            }
            
            let total_by_year: Vec<YearCount> = years_map
                .into_iter()
                .map(|(year, (count, total_weight))| YearCount { 
                    year, 
                    count, 
                    total_weight 
                })
                .collect();
            
            let mut seeds_map: HashMap<String,(usize, i32)> = HashMap::new();
            for batch in &all_batches {
                let entry = seeds_map.entry(batch.seed.clone()).or_insert((0, 0));
                entry.0 += 1;
                entry.1 += batch.total_weight;
            }
            
            let total_by_seed: Vec<SeedCount> = seeds_map
                .into_iter()
                .map(|(seed, (count, total_weight))| SeedCount { 
                    seed, 
                    count, 
                    total_weight 
                })
                .collect();
            
            Ok(BatchStatistics {
                total_batches,
                total_weight,
                total_by_year,
                total_by_seed,
            })
        } else {
            return Err("Erro ao obter estatísticas de lotes".to_string())
        }
    }   

    pub fn update(&mut self, id: &(i32, i32), changes: &Batch) -> Result<Batch,String> {
        match self.repo.update(id, changes) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No batch with id {}/{} found", id.0, id.1)),
            Err(e) => Err(e.to_string())
        }
        
    }
    
    pub fn delete(&mut self, id: &(i32, i32)) -> Result<Batch, String> {
        match self.repo.drop(id) {
            Ok(Some(result)) => Ok(result),
            Ok(None) => Err(format!("No batch with id {}/{} found", id.0, id.1)),
            Err(e) => Err(e.to_string())
        }
    }


    fn list_active_batches(&mut self) -> Result<Vec<Batch>, String> {
        
        self.read_all() 
    }

    pub fn generate_selected_batches_pdf(&mut self, selected_ids: Vec<(i32, i32)>, path: String) -> Result<String, String> {
        let batches = self.fetch_selected_batches(selected_ids)?;
        self.generate_pdf_from_batches(&batches, path)
    }

    fn fetch_selected_batches(&mut self, selected_ids: Vec<(i32, i32)>) -> Result<Vec<Batch>, String> {
        if selected_ids.is_empty() {
            self.read_all()
        } else {
            self.repo.select_many(&selected_ids).map_err(|e| e.to_string())
        }
    }

    fn load_font(&mut self) -> Result<FontFamily<FontData>, String> {
        // Caminho absoluto do projeto
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/fonts");

        // let path_str = path.to_str()
        //     .ok_or("Falha ao converter caminho para string")?
        //     .replace("\\", "/");

        // Converte para string e troca todas as barras por barra invertida
        let path_str = path.to_str()
            .ok_or("Falha ao converter caminho para string")?
            .replace("/", "\\");

        println!("Caminho normalizado: {}", path_str);

        from_files(path_str.as_str(), "LiberationSans", None)
            .map_err(|e| format!("Erro ao carregar fonte: {}", e))
    }

    fn generate_pdf_from_batches(&mut self, batches: &[Batch], path: String) -> Result<String, String> {
        if batches.is_empty() {
            return Err("Nenhum lote encontrado para gerar PDF.".to_string());
        }

        let font_family = self.load_font()?;
        
        let mut doc = Document::new(font_family);
        doc.set_title("Relatório dos Lotes");
        // doc.set_margins(Margins::trbl(20, 20, 20, 20));

        doc.push(Paragraph::new(StyledString::new("Relatório de Lotes Selecionados".to_owned(), Style::new().bold().with_font_size(16))).aligned(Alignment::Center));

        let mut table = TableLayout::new(vec![1, 2, 2, 2, 1, 1, 1, 1]);
        // table.set_margins(Margins::trbl(1, 1, 1, 1));

        let header_style = Style::new().bold();
        let mut header_row = table.row();
        header_row.push_element(Paragraph::new(StyledString::new("Lote".to_owned(), header_style.clone())));
        header_row.push_element(Paragraph::new(StyledString::new("Cultivar".to_owned(), header_style.clone())));
        header_row.push_element(Paragraph::new(StyledString::new("Tratamento".to_owned(), header_style.clone())));
        header_row.push_element(Paragraph::new(StyledString::new("Marca".to_owned(), header_style.clone())));
        header_row.push_element(Paragraph::new(StyledString::new("Ano".to_owned(), header_style.clone())));
        header_row.push_element(Paragraph::new(StyledString::new("Sacos".to_owned(), header_style.clone())));
        header_row.push_element(Paragraph::new(StyledString::new("Peso da Sacaria".to_owned(), header_style.clone())));
        header_row.push_element(Paragraph::new(StyledString::new("Quantidade (Kg)".to_owned(), header_style.clone())));
        header_row.push().map_err(|e| format!("Erro ao adicionar cabeçalho: {}", e))?;

        for batch in batches {
            let mut row = table.row();
            row.push_element(Paragraph::new(format!("{}", batch.batch_number)));
            row.push_element(Paragraph::new(batch.seed.clone()));
            row.push_element(Paragraph::new(batch.coating.clone()));
            row.push_element(Paragraph::new(batch.brand.clone()));
            row.push_element(Paragraph::new(format!("{}", batch.batch_year)));
            row.push_element(Paragraph::new(format!("{}", batch.sack_amount)));
            row.push_element(Paragraph::new(format!("{}", batch.sack_weight)));
            row.push_element(Paragraph::new(format!("{}", batch.total_weight)));
            row.push().map_err(|e| format!("Erro ao adicionar linha: {}", e))?;
        }

        doc.push(table);

        let file = std::fs::File::create(&path).map_err(|e| format!("Erro ao criar arquivo PDF: {}", e))?;
        let writer = BufWriter::new(file);
        doc.render(writer).map_err(|e| format!("Erro ao renderizar PDF: {}", e))?;

        Ok(path)
    }

    pub fn get_stock_report(&mut self) -> Result<Vec<StockReportEntry>, String> {
        let active_batches = self.list_active_batches()?;
        let mut stock_map: HashMap<String, (i32, i32)> = HashMap::new();
        
        for batch in active_batches {
            let entry = stock_map.entry(batch.seed.clone()).or_insert((0, 0));
            entry.0 += batch.total_weight; 
            entry.1 += batch.sack_amount; 
        }
        
        let report: Vec<StockReportEntry> = stock_map
            .into_iter()
            .map(|(seed_name, (total_weight, total_sacks))| StockReportEntry {
                seed_name,
                total_weight_in_stock: total_weight,
                total_sacks_in_stock: total_sacks,
            })
            .collect();
            
        Ok(report)
    }
    
    // pub fn get_detailed_batch_report(&mut self) -> Result<Vec<DetailedBatchReportEntry>, String> {
    //     let report_entries = self.list_active_batches()? 
    //         .into_iter()
    //         .map(|batch| {
    //             DetailedBatchReportEntry {
    //                 batch_number: batch.batch_number,
    //                 creation_year: batch.batch_year,
    //                 cultivar: batch.seed, 
    //                 treatment: batch.coating, 
    //                 brand: batch.brand,
    //                 sack_weight: batch.sack_weight,
    //                 sack_amount: batch.sack_amount,
    //                 total_weight: batch.total_weight,
    //                 status: batch.batch_status,
    //                 origin: batch.origin.clone(), 
    //             }
    //         })
    //         .collect();
    //     Ok(report_entries)
    // }

    // pub fn generate_detailed_batch_pdf_report(&mut self) -> Result<String> {
    //     let report_data = self.get_detailed_batch_report()?;
    //     if report_data.is_empty() {
    //         return Err("Nenhum lote ativo encontrado para gerar o relatório.".to_string());
    //     }

    //     let font_family = FontFamily::new_sans_serif();
    //     let mut doc = Document::new(font_family);
    //     doc.set_title("Relatório Detalhado de Lotes");
    //     doc.set_margins(Margins::trbl(20, 20, 20, 20));

    //     doc.push(Paragraph::new("Relatório Detalhado de Lotes").aligned(Alignment::Center).styled(Style::new().bold().with_font_size(16))); 

    //     let mut table = TableLayout::new(vec![1, 2, 2, 2, 1, 1, 1, 1]); // Largura relativa das colunas
    //     table.set_margins(Margins::trbl(1, 1, 1, 1));

    //     // Cabeçalho da tabela
    //     let header_style = Style::new().bold();
    //     let mut header_row = table.row();
    //     header_row.push_element(Paragraph::new("Lote").styled(header_style.clone()));
    //     header_row.push_element(Paragraph::new("Cultivar").styled(header_style.clone()));
    //     header_row.push_element(Paragraph::new("Tratamento").styled(header_style.clone()));
    //     header_row.push_element(Paragraph::new("Marca").styled(header_style.clone()));
    //     header_row.push_element(Paragraph::new("Ano").styled(header_style.clone()));
    //     header_row.push_element(Paragraph::new("Sacos").styled(header_style.clone()));
    //     header_row.push_element(Paragraph::new("Peso Saco").styled(header_style.clone()));
    //     header_row.push_element(Paragraph::new("Peso Total").styled(header_style.clone()));
    //     table.push_row(header_row).map_err(|e| format!("Erro ao adicionar cabeçalho: {}", e))?;

    //     // Adicionar dados dos lotes
    //     for entry in report_data {
    //         let mut row = table.row();
    //         row.push_element(Paragraph::new(format!("{}", entry.batch_number)));
    //         row.push_element(Paragraph::new(entry.cultivar));
    //         row.push_element(Paragraph::new(entry.treatment));
    //         row.push_element(Paragraph::new(entry.brand));
    //         row.push_element(Paragraph::new(format!("{}", entry.creation_year)));
    //         row.push_element(Paragraph::new(format!("{}", entry.sack_amount)));
    //         row.push_element(Paragraph::new(format!("{}", entry.sack_weight)));
    //         row.push_element(Paragraph::new(format!("{}", entry.total_weight)));
    //         table.push_row(row).map_err(|e| format!("Erro ao adicionar linha de dados: {}", e))?;
    //     }

    //     doc.push(table);

    //     let pdf_path = "/tmp/relatorio_detalhado_lotes.pdf".to_string();
        
    //     let file = fs::File::create(&pdf_path).map_err(|e| format!("Erro ao criar arquivo PDF: {}", e))?;
    //     let writer = BufWriter::new(file);
    //     doc.render(writer).map_err(|e| format!("Erro ao renderizar PDF: {}", e))?;

    //     Ok(pdf_path)
    // }
}
