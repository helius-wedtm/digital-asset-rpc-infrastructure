use crate::dao::generated::prelude::AssetData;
use crate::dao::generated::{asset,  asset_data};
use crate::dapi::asset::get_asset_list_data;
use crate::rpc::filter::AssetSorting;
use crate::rpc::response::AssetList;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryOrder, QueryFilter, CursorTrait, ModelTrait, PaginatorTrait};
use sea_orm::DbErr;


pub async fn get_assets_by_owner(
    db: &DatabaseConnection,
    owner_address: Vec<u8>,
    sort_by: AssetSorting,
    limit: u32,
    page: u32,
    before: Vec<u8>,
    after: Vec<u8>,
) -> Result<AssetList, DbErr> {
    let sort_column = match sort_by {
        AssetSorting::Created => asset::Column::CreatedAt,
        AssetSorting::Updated => todo!(),
        AssetSorting::RecentAction => todo!(),
    };

    let assets: Vec<(asset::Model, Option<asset_data::Model>)> = if page > 0 {
        let paginator = asset::Entity::find()
            .filter(asset::Column::Owner.eq(owner_address.clone()))
            .find_also_related(AssetData)
            .order_by_asc(sort_column)
            .paginate(db, limit.try_into().unwrap());

        paginator.fetch_page((page - 1).try_into().unwrap()).await?
    } else if !before.is_empty() {
        let rows = asset::Entity::find()
            .order_by_asc(sort_column)
            .filter(asset::Column::Owner.eq(owner_address.clone()))
            .cursor_by(asset::Column::Id)
            .before(before.clone())
            .first(limit.into())
            .all(db)
            .await?
            .into_iter()
            .map(|x| async move {
                let asset_data = x.find_related(AssetData).one(db).await.unwrap();

                (x, asset_data)
            });

        let assets = futures::future::join_all(rows).await;
        assets
    } else {
        let rows = asset::Entity::find()
            .order_by_asc(sort_column)
            .filter(asset::Column::Owner.eq(owner_address.clone()))
            .cursor_by(asset::Column::Id)
            .after(after.clone())
            .first(limit.into())
            .all(db)
            .await?
            .into_iter()
            .map(|x| async move {
                let asset_data = x.find_related(AssetData).one(db).await.unwrap();

                (x, asset_data)
            });

        let assets = futures::future::join_all(rows).await;
        assets
    };

    let built_assets = get_asset_list_data(db, assets).await?;

    let total = built_assets.len() as u32;

    let page = if page > 0 { Some(page) } else { None };
    let before = if !before.is_empty() {
        Some(String::from_utf8(before).unwrap())
    } else {
        None
    };
    let after = if !after.is_empty() {
        Some(String::from_utf8(after).unwrap())
    } else {
        None
    };

    Ok(AssetList {
        total,
        limit,
        page,
        before,
        after,
        items: built_assets,
    })
}
