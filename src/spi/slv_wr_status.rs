#[doc = "Reader of register SLV_WR_STATUS"]
pub type R = crate::R<u32, super::SLV_WR_STATUS>;
#[doc = "Writer for register SLV_WR_STATUS"]
pub type W = crate::W<u32, super::SLV_WR_STATUS>;
#[doc = "Register SLV_WR_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::SLV_WR_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLV_WR_ST`"]
pub type SLV_WR_ST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLV_WR_ST`"]
pub struct SLV_WR_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_ST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - In the slave mode this register are the status register for the master to write into. In the master mode this register are the higher 32bits in the 64 bits address condition."]
    #[inline(always)]
    pub fn slv_wr_st(&self) -> SLV_WR_ST_R {
        SLV_WR_ST_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - In the slave mode this register are the status register for the master to write into. In the master mode this register are the higher 32bits in the 64 bits address condition."]
    #[inline(always)]
    pub fn slv_wr_st(&mut self) -> SLV_WR_ST_W {
        SLV_WR_ST_W { w: self }
    }
}
