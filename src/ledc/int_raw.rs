#[doc = "Reader of register INT_RAW"]
pub type R = crate::R<u32, super::INT_RAW>;
#[doc = "Writer for register INT_RAW"]
pub type W = crate::W<u32, super::INT_RAW>;
#[doc = "Register INT_RAW `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_RAW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DUTY_CHNG_END_LSCH7_INT_RAW`"]
pub type DUTY_CHNG_END_LSCH7_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_LSCH7_INT_RAW`"]
pub struct DUTY_CHNG_END_LSCH7_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH7_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_LSCH6_INT_RAW`"]
pub type DUTY_CHNG_END_LSCH6_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_LSCH6_INT_RAW`"]
pub struct DUTY_CHNG_END_LSCH6_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH6_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_LSCH5_INT_RAW`"]
pub type DUTY_CHNG_END_LSCH5_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_LSCH5_INT_RAW`"]
pub struct DUTY_CHNG_END_LSCH5_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH5_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_LSCH4_INT_RAW`"]
pub type DUTY_CHNG_END_LSCH4_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_LSCH4_INT_RAW`"]
pub struct DUTY_CHNG_END_LSCH4_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH4_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_LSCH3_INT_RAW`"]
pub type DUTY_CHNG_END_LSCH3_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_LSCH3_INT_RAW`"]
pub struct DUTY_CHNG_END_LSCH3_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH3_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_LSCH2_INT_RAW`"]
pub type DUTY_CHNG_END_LSCH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_LSCH2_INT_RAW`"]
pub struct DUTY_CHNG_END_LSCH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_LSCH1_INT_RAW`"]
pub type DUTY_CHNG_END_LSCH1_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_LSCH1_INT_RAW`"]
pub struct DUTY_CHNG_END_LSCH1_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH1_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_LSCH0_INT_RAW`"]
pub type DUTY_CHNG_END_LSCH0_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_LSCH0_INT_RAW`"]
pub struct DUTY_CHNG_END_LSCH0_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH0_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_HSCH7_INT_RAW`"]
pub type DUTY_CHNG_END_HSCH7_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_HSCH7_INT_RAW`"]
pub struct DUTY_CHNG_END_HSCH7_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH7_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_HSCH6_INT_RAW`"]
pub type DUTY_CHNG_END_HSCH6_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_HSCH6_INT_RAW`"]
pub struct DUTY_CHNG_END_HSCH6_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH6_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_HSCH5_INT_RAW`"]
pub type DUTY_CHNG_END_HSCH5_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_HSCH5_INT_RAW`"]
pub struct DUTY_CHNG_END_HSCH5_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH5_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_HSCH4_INT_RAW`"]
pub type DUTY_CHNG_END_HSCH4_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_HSCH4_INT_RAW`"]
pub struct DUTY_CHNG_END_HSCH4_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH4_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_HSCH3_INT_RAW`"]
pub type DUTY_CHNG_END_HSCH3_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_HSCH3_INT_RAW`"]
pub struct DUTY_CHNG_END_HSCH3_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH3_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_HSCH2_INT_RAW`"]
pub type DUTY_CHNG_END_HSCH2_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_HSCH2_INT_RAW`"]
pub struct DUTY_CHNG_END_HSCH2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH2_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_HSCH1_INT_RAW`"]
pub type DUTY_CHNG_END_HSCH1_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_HSCH1_INT_RAW`"]
pub struct DUTY_CHNG_END_HSCH1_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH1_INT_RAW_W<'a> {
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
#[doc = "Reader of field `DUTY_CHNG_END_HSCH0_INT_RAW`"]
pub type DUTY_CHNG_END_HSCH0_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUTY_CHNG_END_HSCH0_INT_RAW`"]
pub struct DUTY_CHNG_END_HSCH0_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_HSCH0_INT_RAW_W<'a> {
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
#[doc = "Reader of field `LSTIMER3_OVF_INT_RAW`"]
pub type LSTIMER3_OVF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSTIMER3_OVF_INT_RAW`"]
pub struct LSTIMER3_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER3_OVF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `LSTIMER2_OVF_INT_RAW`"]
pub type LSTIMER2_OVF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSTIMER2_OVF_INT_RAW`"]
pub struct LSTIMER2_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER2_OVF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `LSTIMER1_OVF_INT_RAW`"]
pub type LSTIMER1_OVF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSTIMER1_OVF_INT_RAW`"]
pub struct LSTIMER1_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER1_OVF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `LSTIMER0_OVF_INT_RAW`"]
pub type LSTIMER0_OVF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSTIMER0_OVF_INT_RAW`"]
pub struct LSTIMER0_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER0_OVF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `HSTIMER3_OVF_INT_RAW`"]
pub type HSTIMER3_OVF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTIMER3_OVF_INT_RAW`"]
pub struct HSTIMER3_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER3_OVF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `HSTIMER2_OVF_INT_RAW`"]
pub type HSTIMER2_OVF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTIMER2_OVF_INT_RAW`"]
pub struct HSTIMER2_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER2_OVF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `HSTIMER1_OVF_INT_RAW`"]
pub type HSTIMER1_OVF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTIMER1_OVF_INT_RAW`"]
pub struct HSTIMER1_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER1_OVF_INT_RAW_W<'a> {
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
#[doc = "Reader of field `HSTIMER0_OVF_INT_RAW`"]
pub type HSTIMER0_OVF_INT_RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSTIMER0_OVF_INT_RAW`"]
pub struct HSTIMER0_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTIMER0_OVF_INT_RAW_W<'a> {
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
    #[doc = "Bit 23 - The interrupt raw bit for low speed channel 7 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch7_int_raw(&self) -> DUTY_CHNG_END_LSCH7_INT_RAW_R {
        DUTY_CHNG_END_LSCH7_INT_RAW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - The interrupt raw bit for low speed channel 6 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch6_int_raw(&self) -> DUTY_CHNG_END_LSCH6_INT_RAW_R {
        DUTY_CHNG_END_LSCH6_INT_RAW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - The interrupt raw bit for low speed channel 5 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_raw(&self) -> DUTY_CHNG_END_LSCH5_INT_RAW_R {
        DUTY_CHNG_END_LSCH5_INT_RAW_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - The interrupt raw bit for low speed channel 4 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_raw(&self) -> DUTY_CHNG_END_LSCH4_INT_RAW_R {
        DUTY_CHNG_END_LSCH4_INT_RAW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The interrupt raw bit for low speed channel 3 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_raw(&self) -> DUTY_CHNG_END_LSCH3_INT_RAW_R {
        DUTY_CHNG_END_LSCH3_INT_RAW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - The interrupt raw bit for low speed channel 2 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_raw(&self) -> DUTY_CHNG_END_LSCH2_INT_RAW_R {
        DUTY_CHNG_END_LSCH2_INT_RAW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - The interrupt raw bit for low speed channel 1 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_raw(&self) -> DUTY_CHNG_END_LSCH1_INT_RAW_R {
        DUTY_CHNG_END_LSCH1_INT_RAW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - The interrupt raw bit for low speed channel 0 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_raw(&self) -> DUTY_CHNG_END_LSCH0_INT_RAW_R {
        DUTY_CHNG_END_LSCH0_INT_RAW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - The interrupt raw bit for high speed channel 7 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch7_int_raw(&self) -> DUTY_CHNG_END_HSCH7_INT_RAW_R {
        DUTY_CHNG_END_HSCH7_INT_RAW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - The interrupt raw bit for high speed channel 6 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch6_int_raw(&self) -> DUTY_CHNG_END_HSCH6_INT_RAW_R {
        DUTY_CHNG_END_HSCH6_INT_RAW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - The interrupt raw bit for high speed channel 5 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch5_int_raw(&self) -> DUTY_CHNG_END_HSCH5_INT_RAW_R {
        DUTY_CHNG_END_HSCH5_INT_RAW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - The interrupt raw bit for high speed channel 4 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch4_int_raw(&self) -> DUTY_CHNG_END_HSCH4_INT_RAW_R {
        DUTY_CHNG_END_HSCH4_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - The interrupt raw bit for high speed channel 3 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch3_int_raw(&self) -> DUTY_CHNG_END_HSCH3_INT_RAW_R {
        DUTY_CHNG_END_HSCH3_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - The interrupt raw bit for high speed channel 2 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch2_int_raw(&self) -> DUTY_CHNG_END_HSCH2_INT_RAW_R {
        DUTY_CHNG_END_HSCH2_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - The interrupt raw bit for high speed channel 1 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch1_int_raw(&self) -> DUTY_CHNG_END_HSCH1_INT_RAW_R {
        DUTY_CHNG_END_HSCH1_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - The interrupt raw bit for high speed channel 0 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch0_int_raw(&self) -> DUTY_CHNG_END_HSCH0_INT_RAW_R {
        DUTY_CHNG_END_HSCH0_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for low speed channel3 counter overflow."]
    #[inline(always)]
    pub fn lstimer3_ovf_int_raw(&self) -> LSTIMER3_OVF_INT_RAW_R {
        LSTIMER3_OVF_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - The interrupt raw bit for low speed channel2 counter overflow."]
    #[inline(always)]
    pub fn lstimer2_ovf_int_raw(&self) -> LSTIMER2_OVF_INT_RAW_R {
        LSTIMER2_OVF_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for low speed channel1 counter overflow."]
    #[inline(always)]
    pub fn lstimer1_ovf_int_raw(&self) -> LSTIMER1_OVF_INT_RAW_R {
        LSTIMER1_OVF_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for low speed channel0 counter overflow."]
    #[inline(always)]
    pub fn lstimer0_ovf_int_raw(&self) -> LSTIMER0_OVF_INT_RAW_R {
        LSTIMER0_OVF_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for high speed channel3 counter overflow."]
    #[inline(always)]
    pub fn hstimer3_ovf_int_raw(&self) -> HSTIMER3_OVF_INT_RAW_R {
        HSTIMER3_OVF_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for high speed channel2 counter overflow."]
    #[inline(always)]
    pub fn hstimer2_ovf_int_raw(&self) -> HSTIMER2_OVF_INT_RAW_R {
        HSTIMER2_OVF_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for high speed channel1 counter overflow."]
    #[inline(always)]
    pub fn hstimer1_ovf_int_raw(&self) -> HSTIMER1_OVF_INT_RAW_R {
        HSTIMER1_OVF_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The interrupt raw bit for high speed channel0 counter overflow."]
    #[inline(always)]
    pub fn hstimer0_ovf_int_raw(&self) -> HSTIMER0_OVF_INT_RAW_R {
        HSTIMER0_OVF_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - The interrupt raw bit for low speed channel 7 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch7_int_raw(&mut self) -> DUTY_CHNG_END_LSCH7_INT_RAW_W {
        DUTY_CHNG_END_LSCH7_INT_RAW_W { w: self }
    }
    #[doc = "Bit 22 - The interrupt raw bit for low speed channel 6 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch6_int_raw(&mut self) -> DUTY_CHNG_END_LSCH6_INT_RAW_W {
        DUTY_CHNG_END_LSCH6_INT_RAW_W { w: self }
    }
    #[doc = "Bit 21 - The interrupt raw bit for low speed channel 5 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_raw(&mut self) -> DUTY_CHNG_END_LSCH5_INT_RAW_W {
        DUTY_CHNG_END_LSCH5_INT_RAW_W { w: self }
    }
    #[doc = "Bit 20 - The interrupt raw bit for low speed channel 4 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_raw(&mut self) -> DUTY_CHNG_END_LSCH4_INT_RAW_W {
        DUTY_CHNG_END_LSCH4_INT_RAW_W { w: self }
    }
    #[doc = "Bit 19 - The interrupt raw bit for low speed channel 3 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_raw(&mut self) -> DUTY_CHNG_END_LSCH3_INT_RAW_W {
        DUTY_CHNG_END_LSCH3_INT_RAW_W { w: self }
    }
    #[doc = "Bit 18 - The interrupt raw bit for low speed channel 2 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_raw(&mut self) -> DUTY_CHNG_END_LSCH2_INT_RAW_W {
        DUTY_CHNG_END_LSCH2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 17 - The interrupt raw bit for low speed channel 1 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_raw(&mut self) -> DUTY_CHNG_END_LSCH1_INT_RAW_W {
        DUTY_CHNG_END_LSCH1_INT_RAW_W { w: self }
    }
    #[doc = "Bit 16 - The interrupt raw bit for low speed channel 0 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_raw(&mut self) -> DUTY_CHNG_END_LSCH0_INT_RAW_W {
        DUTY_CHNG_END_LSCH0_INT_RAW_W { w: self }
    }
    #[doc = "Bit 15 - The interrupt raw bit for high speed channel 7 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch7_int_raw(&mut self) -> DUTY_CHNG_END_HSCH7_INT_RAW_W {
        DUTY_CHNG_END_HSCH7_INT_RAW_W { w: self }
    }
    #[doc = "Bit 14 - The interrupt raw bit for high speed channel 6 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch6_int_raw(&mut self) -> DUTY_CHNG_END_HSCH6_INT_RAW_W {
        DUTY_CHNG_END_HSCH6_INT_RAW_W { w: self }
    }
    #[doc = "Bit 13 - The interrupt raw bit for high speed channel 5 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch5_int_raw(&mut self) -> DUTY_CHNG_END_HSCH5_INT_RAW_W {
        DUTY_CHNG_END_HSCH5_INT_RAW_W { w: self }
    }
    #[doc = "Bit 12 - The interrupt raw bit for high speed channel 4 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch4_int_raw(&mut self) -> DUTY_CHNG_END_HSCH4_INT_RAW_W {
        DUTY_CHNG_END_HSCH4_INT_RAW_W { w: self }
    }
    #[doc = "Bit 11 - The interrupt raw bit for high speed channel 3 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch3_int_raw(&mut self) -> DUTY_CHNG_END_HSCH3_INT_RAW_W {
        DUTY_CHNG_END_HSCH3_INT_RAW_W { w: self }
    }
    #[doc = "Bit 10 - The interrupt raw bit for high speed channel 2 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch2_int_raw(&mut self) -> DUTY_CHNG_END_HSCH2_INT_RAW_W {
        DUTY_CHNG_END_HSCH2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 9 - The interrupt raw bit for high speed channel 1 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch1_int_raw(&mut self) -> DUTY_CHNG_END_HSCH1_INT_RAW_W {
        DUTY_CHNG_END_HSCH1_INT_RAW_W { w: self }
    }
    #[doc = "Bit 8 - The interrupt raw bit for high speed channel 0 duty change done."]
    #[inline(always)]
    pub fn duty_chng_end_hsch0_int_raw(&mut self) -> DUTY_CHNG_END_HSCH0_INT_RAW_W {
        DUTY_CHNG_END_HSCH0_INT_RAW_W { w: self }
    }
    #[doc = "Bit 7 - The interrupt raw bit for low speed channel3 counter overflow."]
    #[inline(always)]
    pub fn lstimer3_ovf_int_raw(&mut self) -> LSTIMER3_OVF_INT_RAW_W {
        LSTIMER3_OVF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 6 - The interrupt raw bit for low speed channel2 counter overflow."]
    #[inline(always)]
    pub fn lstimer2_ovf_int_raw(&mut self) -> LSTIMER2_OVF_INT_RAW_W {
        LSTIMER2_OVF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 5 - The interrupt raw bit for low speed channel1 counter overflow."]
    #[inline(always)]
    pub fn lstimer1_ovf_int_raw(&mut self) -> LSTIMER1_OVF_INT_RAW_W {
        LSTIMER1_OVF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 4 - The interrupt raw bit for low speed channel0 counter overflow."]
    #[inline(always)]
    pub fn lstimer0_ovf_int_raw(&mut self) -> LSTIMER0_OVF_INT_RAW_W {
        LSTIMER0_OVF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 3 - The interrupt raw bit for high speed channel3 counter overflow."]
    #[inline(always)]
    pub fn hstimer3_ovf_int_raw(&mut self) -> HSTIMER3_OVF_INT_RAW_W {
        HSTIMER3_OVF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 2 - The interrupt raw bit for high speed channel2 counter overflow."]
    #[inline(always)]
    pub fn hstimer2_ovf_int_raw(&mut self) -> HSTIMER2_OVF_INT_RAW_W {
        HSTIMER2_OVF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1 - The interrupt raw bit for high speed channel1 counter overflow."]
    #[inline(always)]
    pub fn hstimer1_ovf_int_raw(&mut self) -> HSTIMER1_OVF_INT_RAW_W {
        HSTIMER1_OVF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0 - The interrupt raw bit for high speed channel0 counter overflow."]
    #[inline(always)]
    pub fn hstimer0_ovf_int_raw(&mut self) -> HSTIMER0_OVF_INT_RAW_W {
        HSTIMER0_OVF_INT_RAW_W { w: self }
    }
}
