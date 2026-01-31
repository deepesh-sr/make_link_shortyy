use metrics::counter;

pub fn internal_error<E>(error:E) -> (StatusCode, String)
where 
    E : std::error::Error
    {
        tracing::error!("{}",error);
        let label = [("error",format!("{}",error))];
    }
