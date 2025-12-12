use ort::session::{builder::GraphOptimizationLevel, Session};
use ort::value::Tensor;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Ort(ort::Error),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<ort::Error> for AppError {
    fn from(e: ort::Error) -> Self {
        AppError::Ort(e)
    }
}

pub struct Models {
    pub tcn_model: Session,
    pub xgb_model: Session,
}

static mut MODELS: Option<Models> = None;

fn init_model(model_path: &str) -> Result<Session, AppError> {
    let model_data = std::fs::read(model_path)?;
    let session = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(1)?
        .commit_from_memory(&model_data)?;

    Ok(session)
}

fn run_model(
    tcn: &mut Session,
    xgboost: &mut Session,
    mut input: Vec<f32>,
    shape_tcn: [usize; 3],
    shape_xgb: [usize; 2],
) -> Result<Vec<f32>, ort::Error> {
    let eps = 1e-12;

    let min = input.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max = input.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let range = max - min;

    if range < eps {
        for v in &mut input {
            *v = 0.0;
        }
    } else {
        for v in &mut input {
            *v = (*v - min) / (range + eps);
        }
    }

    let tensor_tcn: Tensor<f32> = Tensor::from_array((shape_tcn, input.clone()))?;
    let tensor_xgb: Tensor<f32> = Tensor::from_array((shape_xgb, input))?;

    let t_in: String = tcn.inputs[0].name.clone();
    let t_out = tcn.outputs[0].name.clone();
    let x_in = xgboost.inputs[0].name.clone();
    let x_out = xgboost.outputs[0].name.clone();

    let t_outputs = tcn.run(ort::inputs![t_in => tensor_tcn])?;
    let t_arr = t_outputs[t_out].try_extract_array::<f32>()?;
    let mut t_pred = t_arr.iter().copied().collect::<Vec<f32>>();

    let x_outputs = xgboost.run(ort::inputs![x_in => tensor_xgb])?;
    let x_arr = x_outputs[x_out].try_extract_array::<f32>()?;
    let x_pred = x_arr.iter().copied().collect::<Vec<f32>>();

    for (tp, xp) in t_pred.iter_mut().zip(x_pred.iter()) {
        *tp += *xp;
    }

    if range < eps {
        for v in &mut t_pred {
            *v = min;
        }
    } else {
        for v in &mut t_pred {
            *v = *v * (range + eps) + min;
        }
    }

    Ok(t_pred)
}

pub fn predict(inputs: Vec<[u64; 50]>) -> Result<Vec<u64>, ort::Error> {
    let n = 16;
    let models: &mut Models = unsafe { MODELS.as_mut().unwrap() };
    let tcn: &mut Session = &mut models.tcn_model;
    let xgboost: &mut Session = &mut models.xgb_model;

    let batch = &inputs[..n.min(inputs.len())];

    let input_f32: Vec<f32> = batch
        .iter()
        .flat_map(|arr| arr.iter().map(|&x| x as f32 / 1000000.0))
        .collect();

    let preds = run_model(
        tcn,
        xgboost,
        input_f32,
        [batch.len(), 50, 1],
        [batch.len(), 50],
    )?;

    let out: Vec<u64> = preds.iter().map(|&v| (v * 1000000.0) as u64).collect();

    Ok(out)
}

pub fn init() -> Result<(), AppError> {
    let tcn_path = "/home/deniz/scx-pardus/scheds/rust/scx_pardus/src/res/model.onnx";
    let xgboost_path = "/home/deniz/scx-pardus/scheds/rust/scx_pardus/src/res/xgboost.onnx";

    let tcn_model: Session = init_model(tcn_path)?;
    let xgb_model: Session = init_model(xgboost_path)?;

    unsafe {
        MODELS = Some(Models {
            tcn_model: tcn_model,
            xgb_model: xgb_model,
        });
    }

    /*
    let n = 64;
    let mut inputs: Vec<[u64; 50]> = Vec::with_capacity(n);

    for i in 0..n {
        let mut window = [0u64; 50];
        for j in 0..50 {
            window[j] = (i * 50 + j) as u64;
        }
        inputs.push(window);
    }

    let preds = predict(&mut tcn_model, &mut xgb_model, inputs, n)?;


    for (i, p) in preds.iter().enumerate() {
        println!("Batch {}: Pred = {}", i, p);
    }
    */

    Ok(())
}
