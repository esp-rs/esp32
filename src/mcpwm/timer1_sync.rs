#[doc = "Reader of register TIMER1_SYNC"]
pub type R = crate::R<u32, super::TIMER1_SYNC>;
#[doc = "Writer for register TIMER1_SYNC"]
pub type W = crate::W<u32, super::TIMER1_SYNC>;
#[doc = "Register TIMER1_SYNC `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMER1_SYNC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPWM_TIMER1_PHASE`"]
pub type MCPWM_TIMER1_PHASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MCPWM_TIMER1_PHASE`"]
pub struct MCPWM_TIMER1_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER1_PHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 4)) | (((value as u32) & 0x0001_ffff) << 4);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_TIMER1_SYNCO_SEL`"]
pub type MCPWM_TIMER1_SYNCO_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPWM_TIMER1_SYNCO_SEL`"]
pub struct MCPWM_TIMER1_SYNCO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER1_SYNCO_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `MCPWM_TIMER1_SYNC_SW`"]
pub type MCPWM_TIMER1_SYNC_SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_TIMER1_SYNC_SW`"]
pub struct MCPWM_TIMER1_SYNC_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER1_SYNC_SW_W<'a> {
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
#[doc = "Reader of field `MCPWM_TIMER1_SYNCI_EN`"]
pub type MCPWM_TIMER1_SYNCI_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCPWM_TIMER1_SYNCI_EN`"]
pub struct MCPWM_TIMER1_SYNCI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPWM_TIMER1_SYNCI_EN_W<'a> {
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
    #[doc = "Bits 4:20 - Phase for timer reload on sync event"]
    #[inline(always)]
    pub fn mcpwm_timer1_phase(&self) -> MCPWM_TIMER1_PHASE_R {
        MCPWM_TIMER1_PHASE_R::new(((self.bits >> 4) & 0x0001_ffff) as u32)
    }
    #[doc = "Bits 2:3 - PWM timer1 synco selection 0: synci 1: TEZ 2: TEP else 0"]
    #[inline(always)]
    pub fn mcpwm_timer1_synco_sel(&self) -> MCPWM_TIMER1_SYNCO_SEL_R {
        MCPWM_TIMER1_SYNCO_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - Toggling this bit will trigger a software sync"]
    #[inline(always)]
    pub fn mcpwm_timer1_sync_sw(&self) -> MCPWM_TIMER1_SYNC_SW_R {
        MCPWM_TIMER1_SYNC_SW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - When set timer reload with phase on sync input event is enabled"]
    #[inline(always)]
    pub fn mcpwm_timer1_synci_en(&self) -> MCPWM_TIMER1_SYNCI_EN_R {
        MCPWM_TIMER1_SYNCI_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:20 - Phase for timer reload on sync event"]
    #[inline(always)]
    pub fn mcpwm_timer1_phase(&mut self) -> MCPWM_TIMER1_PHASE_W {
        MCPWM_TIMER1_PHASE_W { w: self }
    }
    #[doc = "Bits 2:3 - PWM timer1 synco selection 0: synci 1: TEZ 2: TEP else 0"]
    #[inline(always)]
    pub fn mcpwm_timer1_synco_sel(&mut self) -> MCPWM_TIMER1_SYNCO_SEL_W {
        MCPWM_TIMER1_SYNCO_SEL_W { w: self }
    }
    #[doc = "Bit 1 - Toggling this bit will trigger a software sync"]
    #[inline(always)]
    pub fn mcpwm_timer1_sync_sw(&mut self) -> MCPWM_TIMER1_SYNC_SW_W {
        MCPWM_TIMER1_SYNC_SW_W { w: self }
    }
    #[doc = "Bit 0 - When set timer reload with phase on sync input event is enabled"]
    #[inline(always)]
    pub fn mcpwm_timer1_synci_en(&mut self) -> MCPWM_TIMER1_SYNCI_EN_W {
        MCPWM_TIMER1_SYNCI_EN_W { w: self }
    }
}
