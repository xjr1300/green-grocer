pub mod vegetable_repository;

use self::vegetable_repository::VegetableRepository;

/// リポジトリコンテナ
#[derive(Clone, Debug)]
pub struct RepositoryContainer<VR>
where
    VR: VegetableRepository,
{
    /// 野菜リポジトリ
    pub vegetable: VR,
}
