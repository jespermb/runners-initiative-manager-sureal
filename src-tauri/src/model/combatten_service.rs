use async_trait::async_trait;
use serde_json::Value;
use crate::{
    repository::repository::Repository, 
    model::combatten::Combatten, 
    action_handler::{ActionHandler, ActionDispatcher}, 
    actions::{combatten_action::{COMBATTEN_DOMAIN, CombattenAction, EditNameDto}, 
    application_action::{ApplicationAction, CombattenDto}}
};

pub struct CombattenService {
    repository : Box<dyn Repository<Combatten> + Send + Sync>
}
impl CombattenService {
    
    pub fn new(repository: Box<dyn Repository<Combatten> + Send + Sync>) -> Self { 
        Self { repository }
    }

    pub async fn load_combattens(&self) -> Vec<Combatten> {
        let combattens = self.repository.query_all().await;
        combattens
    }

    pub async fn get_by_id(&self, id: &str) -> Combatten {
        self.repository.query_by_id(id).await.unwrap()
    }

    pub async fn create_new_combatten(&self, new_name: &str) -> Combatten {
        let new_combatten = self.repository.insert(Combatten{
            name: new_name.to_string(),
            ..Default::default()
        }).await;
        new_combatten
    }
    
    pub async fn update_combatten_name(&self, id: &str, new_name: &str) -> Combatten {
        let mut combatten = self.repository.query_by_id(id).await.unwrap();
        combatten.name = new_name.to_string();
        // a bit ugly, but we need to copy the id because "edit" owns the containing struct
        let id = combatten.id.clone();
        let updated = self.repository.edit(&id.as_ref().unwrap(), combatten).await;
        updated
    }
}

#[async_trait]
impl ActionDispatcher for CombattenService {
    async fn dispatch_action(&self, domain: String, action: Value) ->  Value {
        if domain == COMBATTEN_DOMAIN {
            ActionHandler::<CombattenAction>::convert_and_handle(self, action).await
        } else {
            ActionHandler::<ApplicationAction>::convert_and_handle(self, action).await
        }
    }
}

#[async_trait]
impl ActionHandler<CombattenAction> for CombattenService {
    async fn handle_action(&self, action: CombattenAction) -> CombattenAction {
        let response = match action {
          CombattenAction::RenameCombatten(data) => {
                let classifier = self.update_combatten_name(&data.id, &data.new_name).await;
                CombattenAction::CombattenRenamed(
                    EditNameDto{ id: classifier.id.unwrap(), new_name: classifier.name}
                )
            },
            CombattenAction::CancelCombattenRename{id} => {
                let classifier = self.get_by_id(&id).await;
                CombattenAction::CombattenRenameCanceled(
                    EditNameDto { id, new_name: classifier.name }
                )
            },
            _ => CombattenAction::CombattenRenameError
        };
        return response;
    }
}

#[async_trait]
impl ActionHandler<ApplicationAction> for CombattenService {
    async fn handle_action(&self, action: ApplicationAction) -> ApplicationAction {
        let response = match action {
            ApplicationAction::ApplicationReady => {
                // check if there is already a classifier, if not, create one
                let mut combattens = self.load_combattens().await;
                // convert entities to DTOs and return them
                ApplicationAction::CombattenLoaded(
                  combattens
                        .into_iter()
                        .map(|c| CombattenDto{id: c.id.unwrap(), name: c.name})
                        .collect()
                )
            },
            _ => ApplicationAction::ApplicationLoadError
        };
        return response;
    }
}