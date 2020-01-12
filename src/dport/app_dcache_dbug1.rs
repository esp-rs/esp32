#[doc = "Reader of register APP_DCACHE_DBUG1"]
pub type R = crate::R<u32, super::APP_DCACHE_DBUG1>;
#[doc = "Writer for register APP_DCACHE_DBUG1"]
pub type W = crate::W<u32, super::APP_DCACHE_DBUG1>;
#[doc = "Register APP_DCACHE_DBUG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_DCACHE_DBUG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP_CTAG_RAM_RDATA`"]
pub type APP_CTAG_RAM_RDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APP_CTAG_RAM_RDATA`"]
pub struct APP_CTAG_RAM_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CTAG_RAM_RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_ctag_ram_rdata(&self) -> APP_CTAG_RAM_RDATA_R {
        APP_CTAG_RAM_RDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_ctag_ram_rdata(&mut self) -> APP_CTAG_RAM_RDATA_W {
        APP_CTAG_RAM_RDATA_W { w: self }
    }
}
