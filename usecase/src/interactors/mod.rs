pub mod vegetable;

use self::vegetable::VegetableInteractor;

/// ユースケースインタラクターコンテナ
#[derive(Clone, Debug)]
pub struct UsecaseInteractorContainer<VI>
where
    VI: VegetableInteractor,
{
    /// 野菜ユースケースインタラクター
    pub vegetable: VI,
}
