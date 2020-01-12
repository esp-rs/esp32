#[doc = "Reader of register MCMCPWM_INT_ST_MCPWM"]
pub type R = crate::R<u32, super::MCMCPWM_INT_ST_MCPWM>;
#[doc = "Writer for register MCMCPWM_INT_ST_MCPWM"]
pub type W = crate::W<u32, super::MCMCPWM_INT_ST_MCPWM>;
#[doc = "Register MCMCPWM_INT_ST_MCPWM `reset()`'s with value 0"]
impl crate::ResetValue for super::MCMCPWM_INT_ST_MCPWM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAP2_INT_ST`"]
pub type CAP2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP2_INT_ST`"]
pub struct CAP2_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `CAP1_INT_ST`"]
pub type CAP1_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP1_INT_ST`"]
pub struct CAP1_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `CAP0_INT_ST`"]
pub type CAP0_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP0_INT_ST`"]
pub struct CAP0_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `FH2_OST_INT_ST`"]
pub type FH2_OST_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FH2_OST_INT_ST`"]
pub struct FH2_OST_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_OST_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `FH1_OST_INT_ST`"]
pub type FH1_OST_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FH1_OST_INT_ST`"]
pub struct FH1_OST_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> FH1_OST_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `FH0_OST_INT_ST`"]
pub type FH0_OST_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FH0_OST_INT_ST`"]
pub struct FH0_OST_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> FH0_OST_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `FH2_CBC_INT_ST`"]
pub type FH2_CBC_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FH2_CBC_INT_ST`"]
pub struct FH2_CBC_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_CBC_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `FH1_CBC_INT_ST`"]
pub type FH1_CBC_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FH1_CBC_INT_ST`"]
pub struct FH1_CBC_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> FH1_CBC_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `FH0_CBC_INT_ST`"]
pub type FH0_CBC_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FH0_CBC_INT_ST`"]
pub struct FH0_CBC_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> FH0_CBC_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `OP2_TEB_INT_ST`"]
pub type OP2_TEB_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OP2_TEB_INT_ST`"]
pub struct OP2_TEB_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> OP2_TEB_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `OP1_TEB_INT_ST`"]
pub type OP1_TEB_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OP1_TEB_INT_ST`"]
pub struct OP1_TEB_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> OP1_TEB_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `OP0_TEB_INT_ST`"]
pub type OP0_TEB_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OP0_TEB_INT_ST`"]
pub struct OP0_TEB_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> OP0_TEB_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `OP2_TEA_INT_ST`"]
pub type OP2_TEA_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OP2_TEA_INT_ST`"]
pub struct OP2_TEA_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> OP2_TEA_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `OP1_TEA_INT_ST`"]
pub type OP1_TEA_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OP1_TEA_INT_ST`"]
pub struct OP1_TEA_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> OP1_TEA_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `OP0_TEA_INT_ST`"]
pub type OP0_TEA_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OP0_TEA_INT_ST`"]
pub struct OP0_TEA_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> OP0_TEA_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `FAULT2_CLR_INT_ST`"]
pub type FAULT2_CLR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT2_CLR_INT_ST`"]
pub struct FAULT2_CLR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT2_CLR_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `FAULT1_CLR_INT_ST`"]
pub type FAULT1_CLR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT1_CLR_INT_ST`"]
pub struct FAULT1_CLR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT1_CLR_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `FAULT0_CLR_INT_ST`"]
pub type FAULT0_CLR_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT0_CLR_INT_ST`"]
pub struct FAULT0_CLR_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT0_CLR_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `FAULT2_INT_ST`"]
pub type FAULT2_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT2_INT_ST`"]
pub struct FAULT2_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT2_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `FAULT1_INT_ST`"]
pub type FAULT1_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT1_INT_ST`"]
pub struct FAULT1_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT1_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `FAULT0_INT_ST`"]
pub type FAULT0_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAULT0_INT_ST`"]
pub struct FAULT0_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT0_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TIMER2_TEP_INT_ST`"]
pub type TIMER2_TEP_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER2_TEP_INT_ST`"]
pub struct TIMER2_TEP_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_TEP_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TIMER1_TEP_INT_ST`"]
pub type TIMER1_TEP_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER1_TEP_INT_ST`"]
pub struct TIMER1_TEP_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_TEP_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TIMER0_TEP_INT_ST`"]
pub type TIMER0_TEP_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER0_TEP_INT_ST`"]
pub struct TIMER0_TEP_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_TEP_INT_ST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TIMER2_TEZ_INT_ST`"]
pub type TIMER2_TEZ_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER2_TEZ_INT_ST`"]
pub struct TIMER2_TEZ_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_TEZ_INT_ST_W<'a> {
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
#[doc = "Reader of field `TIMER1_TEZ_INT_ST`"]
pub type TIMER1_TEZ_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER1_TEZ_INT_ST`"]
pub struct TIMER1_TEZ_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_TEZ_INT_ST_W<'a> {
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
#[doc = "Reader of field `TIMER0_TEZ_INT_ST`"]
pub type TIMER0_TEZ_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER0_TEZ_INT_ST`"]
pub struct TIMER0_TEZ_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_TEZ_INT_ST_W<'a> {
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
#[doc = "Reader of field `TIMER2_STOP_INT_ST`"]
pub type TIMER2_STOP_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER2_STOP_INT_ST`"]
pub struct TIMER2_STOP_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_STOP_INT_ST_W<'a> {
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
#[doc = "Reader of field `TIMER1_STOP_INT_ST`"]
pub type TIMER1_STOP_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER1_STOP_INT_ST`"]
pub struct TIMER1_STOP_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_STOP_INT_ST_W<'a> {
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
#[doc = "Reader of field `TIMER0_STOP_INT_ST`"]
pub type TIMER0_STOP_INT_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER0_STOP_INT_ST`"]
pub struct TIMER0_STOP_INT_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_STOP_INT_ST_W<'a> {
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
    #[doc = "Bit 29 - The masked status bit for interrupt triggered by captureon channel 2"]
    #[inline(always)]
    pub fn cap2_int_st(&self) -> CAP2_INT_ST_R {
        CAP2_INT_ST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - The masked status bit for interrupt triggered by captureon channel 1"]
    #[inline(always)]
    pub fn cap1_int_st(&self) -> CAP1_INT_ST_R {
        CAP1_INT_ST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - The masked status bit for interrupt triggered by captureon channel 0"]
    #[inline(always)]
    pub fn cap0_int_st(&self) -> CAP0_INT_ST_R {
        CAP0_INT_ST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - The masked status bit for interrupt triggered by an one-shot mode action on PWM2"]
    #[inline(always)]
    pub fn fh2_ost_int_st(&self) -> FH2_OST_INT_ST_R {
        FH2_OST_INT_ST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - The masked status bit for interrupt triggered by an one-shot mode action on PWM0"]
    #[inline(always)]
    pub fn fh1_ost_int_st(&self) -> FH1_OST_INT_ST_R {
        FH1_OST_INT_ST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - The masked status bit for interrupt triggered by an one-shot mode action on PWM0"]
    #[inline(always)]
    pub fn fh0_ost_int_st(&self) -> FH0_OST_INT_ST_R {
        FH0_OST_INT_ST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - The masked status bit for interrupt triggered by an cycle-by-cycle mode action on PWM2"]
    #[inline(always)]
    pub fn fh2_cbc_int_st(&self) -> FH2_CBC_INT_ST_R {
        FH2_CBC_INT_ST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - The masked status bit for interrupt triggered by an cycle-by-cycle mode action on PWM1"]
    #[inline(always)]
    pub fn fh1_cbc_int_st(&self) -> FH1_CBC_INT_ST_R {
        FH1_CBC_INT_ST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The masked status bit for interrupt triggered by an cycle-by-cycle mode action on PWM0"]
    #[inline(always)]
    pub fn fh0_cbc_int_st(&self) -> FH0_CBC_INT_ST_R {
        FH0_CBC_INT_ST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - The masked status bit for interrupt triggered by a PWM operator 2 TEB event"]
    #[inline(always)]
    pub fn op2_teb_int_st(&self) -> OP2_TEB_INT_ST_R {
        OP2_TEB_INT_ST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The masked status bit for interrupt triggered by a PWM operator 1 TEB event"]
    #[inline(always)]
    pub fn op1_teb_int_st(&self) -> OP1_TEB_INT_ST_R {
        OP1_TEB_INT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - The masked status bit for interrupt triggered by a PWM operator 0 TEB event"]
    #[inline(always)]
    pub fn op0_teb_int_st(&self) -> OP0_TEB_INT_ST_R {
        OP0_TEB_INT_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - The masked status bit for interrupt triggered by a PWM operator 2 TEA event"]
    #[inline(always)]
    pub fn op2_tea_int_st(&self) -> OP2_TEA_INT_ST_R {
        OP2_TEA_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - The masked status bit for interrupt triggered by a PWM operator 1 TEA event"]
    #[inline(always)]
    pub fn op1_tea_int_st(&self) -> OP1_TEA_INT_ST_R {
        OP1_TEA_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - The masked status bit for interrupt triggered by a PWM operator 0 TEA event"]
    #[inline(always)]
    pub fn op0_tea_int_st(&self) -> OP0_TEA_INT_ST_R {
        OP0_TEA_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The masked status bit for interrupt triggered when event_f2 ends"]
    #[inline(always)]
    pub fn fault2_clr_int_st(&self) -> FAULT2_CLR_INT_ST_R {
        FAULT2_CLR_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The masked status bit for interrupt triggered when event_f1 ends"]
    #[inline(always)]
    pub fn fault1_clr_int_st(&self) -> FAULT1_CLR_INT_ST_R {
        FAULT1_CLR_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The masked status bit for interrupt triggered when event_f0 ends"]
    #[inline(always)]
    pub fn fault0_clr_int_st(&self) -> FAULT0_CLR_INT_ST_R {
        FAULT0_CLR_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The masked status bit for interrupt triggered when event_f2 starts"]
    #[inline(always)]
    pub fn fault2_int_st(&self) -> FAULT2_INT_ST_R {
        FAULT2_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The masked status bit for interrupt triggered when event_f1 starts"]
    #[inline(always)]
    pub fn fault1_int_st(&self) -> FAULT1_INT_ST_R {
        FAULT1_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The masked status bit for interrupt triggered when event_f0 starts"]
    #[inline(always)]
    pub fn fault0_int_st(&self) -> FAULT0_INT_ST_R {
        FAULT0_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The masked status bit for interrupt triggered by a PWM timer 2 TEP event"]
    #[inline(always)]
    pub fn timer2_tep_int_st(&self) -> TIMER2_TEP_INT_ST_R {
        TIMER2_TEP_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The masked status bit for interrupt triggered by a PWM timer 1 TEP event"]
    #[inline(always)]
    pub fn timer1_tep_int_st(&self) -> TIMER1_TEP_INT_ST_R {
        TIMER1_TEP_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The masked status bit for interrupt triggered by a PWM timer 0 TEP event"]
    #[inline(always)]
    pub fn timer0_tep_int_st(&self) -> TIMER0_TEP_INT_ST_R {
        TIMER0_TEP_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The masked status bit for interrupt triggered by a PWM timer 2 TEZ event"]
    #[inline(always)]
    pub fn timer2_tez_int_st(&self) -> TIMER2_TEZ_INT_ST_R {
        TIMER2_TEZ_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The masked status bit for interrupt triggered by a PWM timer 1 TEZ event"]
    #[inline(always)]
    pub fn timer1_tez_int_st(&self) -> TIMER1_TEZ_INT_ST_R {
        TIMER1_TEZ_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The masked status bit for interrupt triggered by a PWM timer 0 TEZ event"]
    #[inline(always)]
    pub fn timer0_tez_int_st(&self) -> TIMER0_TEZ_INT_ST_R {
        TIMER0_TEZ_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The masked status bit for interrupt triggered when timer 2 stops"]
    #[inline(always)]
    pub fn timer2_stop_int_st(&self) -> TIMER2_STOP_INT_ST_R {
        TIMER2_STOP_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - The masked status bit for interrupt triggered when timer 1 stops"]
    #[inline(always)]
    pub fn timer1_stop_int_st(&self) -> TIMER1_STOP_INT_ST_R {
        TIMER1_STOP_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The masked status bit for interrupt triggered when timer 0 stops"]
    #[inline(always)]
    pub fn timer0_stop_int_st(&self) -> TIMER0_STOP_INT_ST_R {
        TIMER0_STOP_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - The masked status bit for interrupt triggered by captureon channel 2"]
    #[inline(always)]
    pub fn cap2_int_st(&mut self) -> CAP2_INT_ST_W {
        CAP2_INT_ST_W { w: self }
    }
    #[doc = "Bit 28 - The masked status bit for interrupt triggered by captureon channel 1"]
    #[inline(always)]
    pub fn cap1_int_st(&mut self) -> CAP1_INT_ST_W {
        CAP1_INT_ST_W { w: self }
    }
    #[doc = "Bit 27 - The masked status bit for interrupt triggered by captureon channel 0"]
    #[inline(always)]
    pub fn cap0_int_st(&mut self) -> CAP0_INT_ST_W {
        CAP0_INT_ST_W { w: self }
    }
    #[doc = "Bit 26 - The masked status bit for interrupt triggered by an one-shot mode action on PWM2"]
    #[inline(always)]
    pub fn fh2_ost_int_st(&mut self) -> FH2_OST_INT_ST_W {
        FH2_OST_INT_ST_W { w: self }
    }
    #[doc = "Bit 25 - The masked status bit for interrupt triggered by an one-shot mode action on PWM0"]
    #[inline(always)]
    pub fn fh1_ost_int_st(&mut self) -> FH1_OST_INT_ST_W {
        FH1_OST_INT_ST_W { w: self }
    }
    #[doc = "Bit 24 - The masked status bit for interrupt triggered by an one-shot mode action on PWM0"]
    #[inline(always)]
    pub fn fh0_ost_int_st(&mut self) -> FH0_OST_INT_ST_W {
        FH0_OST_INT_ST_W { w: self }
    }
    #[doc = "Bit 23 - The masked status bit for interrupt triggered by an cycle-by-cycle mode action on PWM2"]
    #[inline(always)]
    pub fn fh2_cbc_int_st(&mut self) -> FH2_CBC_INT_ST_W {
        FH2_CBC_INT_ST_W { w: self }
    }
    #[doc = "Bit 22 - The masked status bit for interrupt triggered by an cycle-by-cycle mode action on PWM1"]
    #[inline(always)]
    pub fn fh1_cbc_int_st(&mut self) -> FH1_CBC_INT_ST_W {
        FH1_CBC_INT_ST_W { w: self }
    }
    #[doc = "Bit 21 - The masked status bit for interrupt triggered by an cycle-by-cycle mode action on PWM0"]
    #[inline(always)]
    pub fn fh0_cbc_int_st(&mut self) -> FH0_CBC_INT_ST_W {
        FH0_CBC_INT_ST_W { w: self }
    }
    #[doc = "Bit 20 - The masked status bit for interrupt triggered by a PWM operator 2 TEB event"]
    #[inline(always)]
    pub fn op2_teb_int_st(&mut self) -> OP2_TEB_INT_ST_W {
        OP2_TEB_INT_ST_W { w: self }
    }
    #[doc = "Bit 19 - The masked status bit for interrupt triggered by a PWM operator 1 TEB event"]
    #[inline(always)]
    pub fn op1_teb_int_st(&mut self) -> OP1_TEB_INT_ST_W {
        OP1_TEB_INT_ST_W { w: self }
    }
    #[doc = "Bit 18 - The masked status bit for interrupt triggered by a PWM operator 0 TEB event"]
    #[inline(always)]
    pub fn op0_teb_int_st(&mut self) -> OP0_TEB_INT_ST_W {
        OP0_TEB_INT_ST_W { w: self }
    }
    #[doc = "Bit 17 - The masked status bit for interrupt triggered by a PWM operator 2 TEA event"]
    #[inline(always)]
    pub fn op2_tea_int_st(&mut self) -> OP2_TEA_INT_ST_W {
        OP2_TEA_INT_ST_W { w: self }
    }
    #[doc = "Bit 16 - The masked status bit for interrupt triggered by a PWM operator 1 TEA event"]
    #[inline(always)]
    pub fn op1_tea_int_st(&mut self) -> OP1_TEA_INT_ST_W {
        OP1_TEA_INT_ST_W { w: self }
    }
    #[doc = "Bit 15 - The masked status bit for interrupt triggered by a PWM operator 0 TEA event"]
    #[inline(always)]
    pub fn op0_tea_int_st(&mut self) -> OP0_TEA_INT_ST_W {
        OP0_TEA_INT_ST_W { w: self }
    }
    #[doc = "Bit 14 - The masked status bit for interrupt triggered when event_f2 ends"]
    #[inline(always)]
    pub fn fault2_clr_int_st(&mut self) -> FAULT2_CLR_INT_ST_W {
        FAULT2_CLR_INT_ST_W { w: self }
    }
    #[doc = "Bit 13 - The masked status bit for interrupt triggered when event_f1 ends"]
    #[inline(always)]
    pub fn fault1_clr_int_st(&mut self) -> FAULT1_CLR_INT_ST_W {
        FAULT1_CLR_INT_ST_W { w: self }
    }
    #[doc = "Bit 12 - The masked status bit for interrupt triggered when event_f0 ends"]
    #[inline(always)]
    pub fn fault0_clr_int_st(&mut self) -> FAULT0_CLR_INT_ST_W {
        FAULT0_CLR_INT_ST_W { w: self }
    }
    #[doc = "Bit 11 - The masked status bit for interrupt triggered when event_f2 starts"]
    #[inline(always)]
    pub fn fault2_int_st(&mut self) -> FAULT2_INT_ST_W {
        FAULT2_INT_ST_W { w: self }
    }
    #[doc = "Bit 10 - The masked status bit for interrupt triggered when event_f1 starts"]
    #[inline(always)]
    pub fn fault1_int_st(&mut self) -> FAULT1_INT_ST_W {
        FAULT1_INT_ST_W { w: self }
    }
    #[doc = "Bit 9 - The masked status bit for interrupt triggered when event_f0 starts"]
    #[inline(always)]
    pub fn fault0_int_st(&mut self) -> FAULT0_INT_ST_W {
        FAULT0_INT_ST_W { w: self }
    }
    #[doc = "Bit 8 - The masked status bit for interrupt triggered by a PWM timer 2 TEP event"]
    #[inline(always)]
    pub fn timer2_tep_int_st(&mut self) -> TIMER2_TEP_INT_ST_W {
        TIMER2_TEP_INT_ST_W { w: self }
    }
    #[doc = "Bit 7 - The masked status bit for interrupt triggered by a PWM timer 1 TEP event"]
    #[inline(always)]
    pub fn timer1_tep_int_st(&mut self) -> TIMER1_TEP_INT_ST_W {
        TIMER1_TEP_INT_ST_W { w: self }
    }
    #[doc = "Bit 6 - The masked status bit for interrupt triggered by a PWM timer 0 TEP event"]
    #[inline(always)]
    pub fn timer0_tep_int_st(&mut self) -> TIMER0_TEP_INT_ST_W {
        TIMER0_TEP_INT_ST_W { w: self }
    }
    #[doc = "Bit 5 - The masked status bit for interrupt triggered by a PWM timer 2 TEZ event"]
    #[inline(always)]
    pub fn timer2_tez_int_st(&mut self) -> TIMER2_TEZ_INT_ST_W {
        TIMER2_TEZ_INT_ST_W { w: self }
    }
    #[doc = "Bit 4 - The masked status bit for interrupt triggered by a PWM timer 1 TEZ event"]
    #[inline(always)]
    pub fn timer1_tez_int_st(&mut self) -> TIMER1_TEZ_INT_ST_W {
        TIMER1_TEZ_INT_ST_W { w: self }
    }
    #[doc = "Bit 3 - The masked status bit for interrupt triggered by a PWM timer 0 TEZ event"]
    #[inline(always)]
    pub fn timer0_tez_int_st(&mut self) -> TIMER0_TEZ_INT_ST_W {
        TIMER0_TEZ_INT_ST_W { w: self }
    }
    #[doc = "Bit 2 - The masked status bit for interrupt triggered when timer 2 stops"]
    #[inline(always)]
    pub fn timer2_stop_int_st(&mut self) -> TIMER2_STOP_INT_ST_W {
        TIMER2_STOP_INT_ST_W { w: self }
    }
    #[doc = "Bit 1 - The masked status bit for interrupt triggered when timer 1 stops"]
    #[inline(always)]
    pub fn timer1_stop_int_st(&mut self) -> TIMER1_STOP_INT_ST_W {
        TIMER1_STOP_INT_ST_W { w: self }
    }
    #[doc = "Bit 0 - The masked status bit for interrupt triggered when timer 0 stops"]
    #[inline(always)]
    pub fn timer0_stop_int_st(&mut self) -> TIMER0_STOP_INT_ST_W {
        TIMER0_STOP_INT_ST_W { w: self }
    }
}
