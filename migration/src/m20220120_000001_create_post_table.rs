use entity::*;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220120_000001_create_post_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       _= manager
            .create_table(
                Table::create()
                    .table(post::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(post::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(post::Column::Title).string().not_null())
                    .col(ColumnDef::new(post::Column::Text).string().not_null())
                    .to_owned(),
            )
            .await;
       _= manager
            .create_table(
                Table::create()
                    .table(user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(user::Column::UserName).string().not_null())
                    .col(ColumnDef::new(user::Column::Mobile).text().not_null())
                    .to_owned(),
            )
            .await;

           _= manager
            .create_table(
                Table::create()
                    .table(group::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(group::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(group::Column::Name).string().not_null())
                    .col(ColumnDef::new(group::Column::Des).text())
                    .to_owned(),
            )
            .await;
          _=  manager.create_table(
                Table::create()
                    .table(user_group::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user_group::Column::UserId)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(user_group::Column::GroupId)
                            .integer()
                            .not_null()
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-user-group_userId_groupId")
                            .col(user_group::Column::UserId)
                            .col(user_group::Column::GroupId)
                            .primary(),
                    )
                    .to_owned(),
            )
            .await;

           return manager.create_table(
                Table::create()
                    .table(user_issue::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user_issue::Column::UserId)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(user_issue::Column::IssueId)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(user_issue::Column::Score)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(user_issue::Column::UpdateTime)
                            .date_time()
                    )
                    .col(
                        ColumnDef::new(user_issue::Column::Comment)
                            .string()
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-user-issue_userId_issueId")
                            .col(user_issue::Column::UserId)
                            .col(user_issue::Column::IssueId)
                            .primary(),
                    )
                    .to_owned(),
            )
            .await;
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       _= manager
            .drop_table(Table::drop().table(post::Entity).to_owned())
            .await;
       _= manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await;
      _= manager
            .drop_table(Table::drop().table(group::Entity).to_owned())
            .await;
    _= manager
            .drop_table(Table::drop().table(user_group::Entity).to_owned())
            .await;
    return manager
            .drop_table(Table::drop().table(user_issue::Entity).to_owned())
            .await
    }
}
