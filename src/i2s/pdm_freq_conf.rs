#[doc = "Reader of register PDM_FREQ_CONF"]
pub type R = crate::R<u32, super::PDM_FREQ_CONF>;
#[doc = "Writer for register PDM_FREQ_CONF"]
pub type W = crate::W<u32, super::PDM_FREQ_CONF>;
#[doc = "Register PDM_FREQ_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::PDM_FREQ_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_PDM_FP`"]
pub type TX_PDM_FP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_PDM_FP`"]
pub struct TX_PDM_FP_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_FP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `TX_PDM_FS`"]
pub type TX_PDM_FS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_PDM_FS`"]
pub struct TX_PDM_FS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_FS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn tx_pdm_fp(&self) -> TX_PDM_FP_R {
        TX_PDM_FP_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_pdm_fs(&self) -> TX_PDM_FS_R {
        TX_PDM_FS_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn tx_pdm_fp(&mut self) -> TX_PDM_FP_W {
        TX_PDM_FP_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_pdm_fs(&mut self) -> TX_PDM_FS_W {
        TX_PDM_FS_W { w: self }
    }
}
