#[doc = "Reader of register PRO_DCACHE_DBUG1"]
pub type R = crate::R<u32, super::PRO_DCACHE_DBUG1>;
#[doc = "Writer for register PRO_DCACHE_DBUG1"]
pub type W = crate::W<u32, super::PRO_DCACHE_DBUG1>;
#[doc = "Register PRO_DCACHE_DBUG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_DCACHE_DBUG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRO_CTAG_RAM_RDATA`"]
pub type PRO_CTAG_RAM_RDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PRO_CTAG_RAM_RDATA`"]
pub struct PRO_CTAG_RAM_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CTAG_RAM_RDATA_W<'a> {
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
    pub fn pro_ctag_ram_rdata(&self) -> PRO_CTAG_RAM_RDATA_R {
        PRO_CTAG_RAM_RDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pro_ctag_ram_rdata(&mut self) -> PRO_CTAG_RAM_RDATA_W {
        PRO_CTAG_RAM_RDATA_W { w: self }
    }
}