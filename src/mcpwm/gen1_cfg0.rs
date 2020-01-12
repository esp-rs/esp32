#[doc = "Reader of register GEN1_CFG0"]
pub type R = crate::R<u32, super::GEN1_CFG0>;
#[doc = "Writer for register GEN1_CFG0"]
pub type W = crate::W<u32, super::GEN1_CFG0>;
#[doc = "Register GEN1_CFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::GEN1_CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GEN1_T1_SEL`"]
pub type GEN1_T1_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GEN1_T1_SEL`"]
pub struct GEN1_T1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN1_T1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "Reader of field `GEN1_T0_SEL`"]
pub type GEN1_T0_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GEN1_T0_SEL`"]
pub struct GEN1_T0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN1_T0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `GEN1_CFG_UPMETHOD`"]
pub type GEN1_CFG_UPMETHOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GEN1_CFG_UPMETHOD`"]
pub struct GEN1_CFG_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN1_CFG_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:9 - Source selection for PWM generate1 event_t1 take effect immediately 0: fault_event0 1: fault_event1 2: fault_event2 3: sync_taken 4: none"]
    #[inline(always)]
    pub fn gen1_t1_sel(&self) -> GEN1_T1_SEL_R {
        GEN1_T1_SEL_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Source selection for PWM generate1 event_t0 take effect immediately 0: fault_event0 1: fault_event1 2: fault_event2 3: sync_taken 4: none"]
    #[inline(always)]
    pub fn gen1_t0_sel(&self) -> GEN1_T0_SEL_R {
        GEN1_T0_SEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Update method for PWM generate1's active reg of configuration. 0: immediate bit0: TEZ bit1: TEP bit2: sync. bit3: disable update"]
    #[inline(always)]
    pub fn gen1_cfg_upmethod(&self) -> GEN1_CFG_UPMETHOD_R {
        GEN1_CFG_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:9 - Source selection for PWM generate1 event_t1 take effect immediately 0: fault_event0 1: fault_event1 2: fault_event2 3: sync_taken 4: none"]
    #[inline(always)]
    pub fn gen1_t1_sel(&mut self) -> GEN1_T1_SEL_W {
        GEN1_T1_SEL_W { w: self }
    }
    #[doc = "Bits 4:6 - Source selection for PWM generate1 event_t0 take effect immediately 0: fault_event0 1: fault_event1 2: fault_event2 3: sync_taken 4: none"]
    #[inline(always)]
    pub fn gen1_t0_sel(&mut self) -> GEN1_T0_SEL_W {
        GEN1_T0_SEL_W { w: self }
    }
    #[doc = "Bits 0:3 - Update method for PWM generate1's active reg of configuration. 0: immediate bit0: TEZ bit1: TEP bit2: sync. bit3: disable update"]
    #[inline(always)]
    pub fn gen1_cfg_upmethod(&mut self) -> GEN1_CFG_UPMETHOD_W {
        GEN1_CFG_UPMETHOD_W { w: self }
    }
}