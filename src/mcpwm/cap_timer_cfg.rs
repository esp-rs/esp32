#[doc = "Reader of register CAP_TIMER_CFG"]
pub type R = crate::R<u32, super::CAP_TIMER_CFG>;
#[doc = "Writer for register CAP_TIMER_CFG"]
pub type W = crate::W<u32, super::CAP_TIMER_CFG>;
#[doc = "Register CAP_TIMER_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CAP_TIMER_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_CAP_SYNC_SW`"]
pub type MCPWM_CAP_SYNC_SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_CAP_SYNC_SW`"]
pub struct MCPWM_CAP_SYNC_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CAP_SYNC_SW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_CAP_SYNCI_SEL`"]
pub type MCPWM_CAP_SYNCI_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_CAP_SYNCI_SEL`"]
pub struct MCPWM_CAP_SYNCI_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CAP_SYNCI_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_CAP_SYNCI_EN`"]
pub type MCPWM_CAP_SYNCI_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_CAP_SYNCI_EN`"]
pub struct MCPWM_CAP_SYNCI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CAP_SYNCI_EN_W<'a> {
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
#[doc = "Reader of field `MCPWM_CAP_TIMER_EN`"]
pub type MCPWM_CAP_TIMER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_CAP_TIMER_EN`"]
pub struct MCPWM_CAP_TIMER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_CAP_TIMER_EN_W<'a> {
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
    #[doc = "Bit 5 - Set this bit to force a capture timer sync capture timer is loaded with value in phase register."]
    #[inline(always)]
    pub fn mcpwm_cap_sync_sw(&self) -> MCPWM_CAP_SYNC_SW_R {
        MCPWM_CAP_SYNC_SW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Capture module sync input selection. 0: none 1: timer0 synco 2: timer1 synco 3: timer2 synco 4: SYNC0 from GPIO matrix 5: SYNC1 from GPIO matrix 6: SYNC2 from GPIO matrix"]
    #[inline(always)]
    pub fn mcpwm_cap_synci_sel(&self) -> MCPWM_CAP_SYNCI_SEL_R {
        MCPWM_CAP_SYNCI_SEL_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 1 - When set capture timer sync is enabled."]
    #[inline(always)]
    pub fn mcpwm_cap_synci_en(&self) -> MCPWM_CAP_SYNCI_EN_R {
        MCPWM_CAP_SYNCI_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - When set capture timer incrementing under APB_clk is enabled."]
    #[inline(always)]
    pub fn mcpwm_cap_timer_en(&self) -> MCPWM_CAP_TIMER_EN_R {
        MCPWM_CAP_TIMER_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Set this bit to force a capture timer sync capture timer is loaded with value in phase register."]
    #[inline(always)]
    pub fn mcpwm_cap_sync_sw(&mut self) -> MCPWM_CAP_SYNC_SW_W {
        MCPWM_CAP_SYNC_SW_W { w: self }
    }
    #[doc = "Bits 2:4 - Capture module sync input selection. 0: none 1: timer0 synco 2: timer1 synco 3: timer2 synco 4: SYNC0 from GPIO matrix 5: SYNC1 from GPIO matrix 6: SYNC2 from GPIO matrix"]
    #[inline(always)]
    pub fn mcpwm_cap_synci_sel(&mut self) -> MCPWM_CAP_SYNCI_SEL_W {
        MCPWM_CAP_SYNCI_SEL_W { w: self }
    }
    #[doc = "Bit 1 - When set capture timer sync is enabled."]
    #[inline(always)]
    pub fn mcpwm_cap_synci_en(&mut self) -> MCPWM_CAP_SYNCI_EN_W {
        MCPWM_CAP_SYNCI_EN_W { w: self }
    }
    #[doc = "Bit 0 - When set capture timer incrementing under APB_clk is enabled."]
    #[inline(always)]
    pub fn mcpwm_cap_timer_en(&mut self) -> MCPWM_CAP_TIMER_EN_W {
        MCPWM_CAP_TIMER_EN_W { w: self }
    }
}
