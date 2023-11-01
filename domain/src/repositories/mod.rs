pub mod vegetable;

use self::vegetable::VegetableRepository;

/// リポジトリコンテナ
#[derive(Clone, Debug)]
pub struct RepositoryContainer<VR>
where
    VR: VegetableRepository,
{
    /// 野菜リポジトリ
    pub vegetable: VR,
}
