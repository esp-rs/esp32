#[doc = "Reader of register DPORT_PRO_VECBASE_CTRL_REG"]
pub type R = crate::R<u32, super::DPORT_PRO_VECBASE_CTRL_REG>;
#[doc = "Writer for register DPORT_PRO_VECBASE_CTRL_REG"]
pub type W = crate::W<u32, super::DPORT_PRO_VECBASE_CTRL_REG>;
#[doc = "Register DPORT_PRO_VECBASE_CTRL_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_PRO_VECBASE_CTRL_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_PRO_OUT_VECBASE_SEL`"]
pub type DPORT_PRO_OUT_VECBASE_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_PRO_OUT_VECBASE_SEL`"]
pub struct DPORT_PRO_OUT_VECBASE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_OUT_VECBASE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dport_pro_out_vecbase_sel(&self) -> DPORT_PRO_OUT_VECBASE_SEL_R {
        DPORT_PRO_OUT_VECBASE_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dport_pro_out_vecbase_sel(&mut self) -> DPORT_PRO_OUT_VECBASE_SEL_W {
        DPORT_PRO_OUT_VECBASE_SEL_W { w: self }
    }
}