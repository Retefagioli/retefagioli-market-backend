use async_trait::async_trait;
use actix_web::Error;

#[async_trait]
pub trait CrudService <Key, Element>
{
    async fn save(element: Element) -> Result<(), Error>;
}
