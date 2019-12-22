#[doc = "Reader of register LEDC_LSCH4_CONF0_REG"]
pub type R = crate::R<u32, super::LEDC_LSCH4_CONF0_REG>;
#[doc = "Writer for register LEDC_LSCH4_CONF0_REG"]
pub type W = crate::W<u32, super::LEDC_LSCH4_CONF0_REG>;
#[doc = "Register LEDC_LSCH4_CONF0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_LSCH4_CONF0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_PARA_UP_LSCH4`"]
pub type LEDC_PARA_UP_LSCH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_PARA_UP_LSCH4`"]
pub struct LEDC_PARA_UP_LSCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_PARA_UP_LSCH4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `LEDC_IDLE_LV_LSCH4`"]
pub type LEDC_IDLE_LV_LSCH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_IDLE_LV_LSCH4`"]
pub struct LEDC_IDLE_LV_LSCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_IDLE_LV_LSCH4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `LEDC_SIG_OUT_EN_LSCH4`"]
pub type LEDC_SIG_OUT_EN_LSCH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEDC_SIG_OUT_EN_LSCH4`"]
pub struct LEDC_SIG_OUT_EN_LSCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_SIG_OUT_EN_LSCH4_W<'a> {
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
#[doc = "Reader of field `LEDC_TIMER_SEL_LSCH4`"]
pub type LEDC_TIMER_SEL_LSCH4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEDC_TIMER_SEL_LSCH4`"]
pub struct LEDC_TIMER_SEL_LSCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_TIMER_SEL_LSCH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - This bit is used to update register LEDC_LSCH4_HPOINT and LEDC_LSCH4_DUTY for low speed channel4."]
    #[inline(always)]
    pub fn ledc_para_up_lsch4(&self) -> LEDC_PARA_UP_LSCH4_R {
        LEDC_PARA_UP_LSCH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when low speed channel4 is off."]
    #[inline(always)]
    pub fn ledc_idle_lv_lsch4(&self) -> LEDC_IDLE_LV_LSCH4_R {
        LEDC_IDLE_LV_LSCH4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This is the output enable control bit for low speed channel4."]
    #[inline(always)]
    pub fn ledc_sig_out_en_lsch4(&self) -> LEDC_SIG_OUT_EN_LSCH4_R {
        LEDC_SIG_OUT_EN_LSCH4_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - There are four low speed timers the two bits are used to select one of them for low speed channel4. 2'b00: seletc lstimer0. 2'b01: select lstimer1. 2'b10: select lstimer2. 2'b11: select lstimer3."]
    #[inline(always)]
    pub fn ledc_timer_sel_lsch4(&self) -> LEDC_TIMER_SEL_LSCH4_R {
        LEDC_TIMER_SEL_LSCH4_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - This bit is used to update register LEDC_LSCH4_HPOINT and LEDC_LSCH4_DUTY for low speed channel4."]
    #[inline(always)]
    pub fn ledc_para_up_lsch4(&mut self) -> LEDC_PARA_UP_LSCH4_W {
        LEDC_PARA_UP_LSCH4_W { w: self }
    }
    #[doc = "Bit 3 - This bit is used to control the output value when low speed channel4 is off."]
    #[inline(always)]
    pub fn ledc_idle_lv_lsch4(&mut self) -> LEDC_IDLE_LV_LSCH4_W {
        LEDC_IDLE_LV_LSCH4_W { w: self }
    }
    #[doc = "Bit 2 - This is the output enable control bit for low speed channel4."]
    #[inline(always)]
    pub fn ledc_sig_out_en_lsch4(&mut self) -> LEDC_SIG_OUT_EN_LSCH4_W {
        LEDC_SIG_OUT_EN_LSCH4_W { w: self }
    }
    #[doc = "Bits 0:1 - There are four low speed timers the two bits are used to select one of them for low speed channel4. 2'b00: seletc lstimer0. 2'b01: select lstimer1. 2'b10: select lstimer2. 2'b11: select lstimer3."]
    #[inline(always)]
    pub fn ledc_timer_sel_lsch4(&mut self) -> LEDC_TIMER_SEL_LSCH4_W {
        LEDC_TIMER_SEL_LSCH4_W { w: self }
    }
}
