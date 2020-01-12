#[doc = "Reader of register APP_DCACHE_DBUG9"]
pub type R = crate::R<u32, super::APP_DCACHE_DBUG9>;
#[doc = "Writer for register APP_DCACHE_DBUG9"]
pub type W = crate::W<u32, super::APP_DCACHE_DBUG9>;
#[doc = "Register APP_DCACHE_DBUG9 `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_DCACHE_DBUG9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP_OPSDRAMADDR_IA`"]
pub type APP_OPSDRAMADDR_IA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APP_OPSDRAMADDR_IA`"]
pub struct APP_OPSDRAMADDR_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_OPSDRAMADDR_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn app_opsdramaddr_ia(&self) -> APP_OPSDRAMADDR_IA_R {
        APP_OPSDRAMADDR_IA_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn app_opsdramaddr_ia(&mut self) -> APP_OPSDRAMADDR_IA_W {
        APP_OPSDRAMADDR_IA_W { w: self }
    }
}
