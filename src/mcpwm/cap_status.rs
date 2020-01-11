#[doc = "Reader of register CAP_STATUS"]
pub type R = crate::R<u32, super::CAP_STATUS>;
#[doc = "Writer for register CAP_STATUS"]
pub type W = crate::W<u32, super::CAP_STATUS>;
#[doc = "Register CAP_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::CAP_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_CAP2_EDGE`"]
pub type MCPWM_CAP2_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_CAP2_EDGE`"]
pub struct MCPWM_CAP2_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CAP2_EDGE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_CAP1_EDGE`"]
pub type MCPWM_CAP1_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_CAP1_EDGE`"]
pub struct MCPWM_CAP1_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CAP1_EDGE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_CAP0_EDGE`"]
pub type MCPWM_CAP0_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_CAP0_EDGE`"]
pub struct MCPWM_CAP0_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CAP0_EDGE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Edge of last capture trigger on channel 2 0: posedge 1: negedge"]
    #[inline(always)]
    pub fn mcpwm_cap2_edge(&self) -> MCPWM_CAP2_EDGE_R {
        MCPWM_CAP2_EDGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Edge of last capture trigger on channel 1 0: posedge 1: negedge"]
    #[inline(always)]
    pub fn mcpwm_cap1_edge(&self) -> MCPWM_CAP1_EDGE_R {
        MCPWM_CAP1_EDGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Edge of last capture trigger on channel 0 0: posedge 1: negedge"]
    #[inline(always)]
    pub fn mcpwm_cap0_edge(&self) -> MCPWM_CAP0_EDGE_R {
        MCPWM_CAP0_EDGE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Edge of last capture trigger on channel 2 0: posedge 1: negedge"]
    #[inline(always)]
    pub fn mcpwm_cap2_edge(&mut self) -> MCPWM_CAP2_EDGE_W {
        MCPWM_CAP2_EDGE_W { w: self }
    }
    #[doc = "Bit 1 - Edge of last capture trigger on channel 1 0: posedge 1: negedge"]
    #[inline(always)]
    pub fn mcpwm_cap1_edge(&mut self) -> MCPWM_CAP1_EDGE_W {
        MCPWM_CAP1_EDGE_W { w: self }
    }
    #[doc = "Bit 0 - Edge of last capture trigger on channel 0 0: posedge 1: negedge"]
    #[inline(always)]
    pub fn mcpwm_cap0_edge(&mut self) -> MCPWM_CAP0_EDGE_W {
        MCPWM_CAP0_EDGE_W { w: self }
    }
}
