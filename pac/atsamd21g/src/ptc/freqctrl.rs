#[doc = "Register `FREQCTRL` reader"]
pub type R = crate::R<FreqctrlSpec>;
#[doc = "Register `FREQCTRL` writer"]
pub type W = crate::W<FreqctrlSpec>;
#[doc = "Sample delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Freqhop {
    #[doc = "0: `0`"]
    Freqhop1 = 0,
    #[doc = "1: `1`"]
    Freqhop2 = 1,
    #[doc = "2: `10`"]
    Freqhop3 = 2,
    #[doc = "3: `11`"]
    Freqhop4 = 3,
    #[doc = "4: `100`"]
    Freqhop5 = 4,
    #[doc = "5: `101`"]
    Freqhop6 = 5,
    #[doc = "6: `110`"]
    Freqhop7 = 6,
    #[doc = "7: `111`"]
    Freqhop8 = 7,
    #[doc = "8: `1000`"]
    Freqhop9 = 8,
    #[doc = "9: `1001`"]
    Freqhop10 = 9,
    #[doc = "10: `1010`"]
    Freqhop11 = 10,
    #[doc = "11: `1011`"]
    Freqhop12 = 11,
    #[doc = "12: `1100`"]
    Freqhop13 = 12,
    #[doc = "13: `1101`"]
    Freqhop14 = 13,
    #[doc = "14: `1110`"]
    Freqhop15 = 14,
    #[doc = "15: `1111`"]
    Freqhop16 = 15,
}
impl From<Freqhop> for u8 {
    #[inline(always)]
    fn from(variant: Freqhop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Freqhop {
    type Ux = u8;
}
impl crate::IsEnum for Freqhop {}
#[doc = "Field `SAMPLEDELAY` reader - Sample delay"]
pub type SampledelayR = crate::FieldReader<Freqhop>;
impl SampledelayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Freqhop {
        match self.bits {
            0 => Freqhop::Freqhop1,
            1 => Freqhop::Freqhop2,
            2 => Freqhop::Freqhop3,
            3 => Freqhop::Freqhop4,
            4 => Freqhop::Freqhop5,
            5 => Freqhop::Freqhop6,
            6 => Freqhop::Freqhop7,
            7 => Freqhop::Freqhop8,
            8 => Freqhop::Freqhop9,
            9 => Freqhop::Freqhop10,
            10 => Freqhop::Freqhop11,
            11 => Freqhop::Freqhop12,
            12 => Freqhop::Freqhop13,
            13 => Freqhop::Freqhop14,
            14 => Freqhop::Freqhop15,
            15 => Freqhop::Freqhop16,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_freqhop1(&self) -> bool {
        *self == Freqhop::Freqhop1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_freqhop2(&self) -> bool {
        *self == Freqhop::Freqhop2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_freqhop3(&self) -> bool {
        *self == Freqhop::Freqhop3
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_freqhop4(&self) -> bool {
        *self == Freqhop::Freqhop4
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_freqhop5(&self) -> bool {
        *self == Freqhop::Freqhop5
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_freqhop6(&self) -> bool {
        *self == Freqhop::Freqhop6
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_freqhop7(&self) -> bool {
        *self == Freqhop::Freqhop7
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_freqhop8(&self) -> bool {
        *self == Freqhop::Freqhop8
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_freqhop9(&self) -> bool {
        *self == Freqhop::Freqhop9
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_freqhop10(&self) -> bool {
        *self == Freqhop::Freqhop10
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_freqhop11(&self) -> bool {
        *self == Freqhop::Freqhop11
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_freqhop12(&self) -> bool {
        *self == Freqhop::Freqhop12
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_freqhop13(&self) -> bool {
        *self == Freqhop::Freqhop13
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_freqhop14(&self) -> bool {
        *self == Freqhop::Freqhop14
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_freqhop15(&self) -> bool {
        *self == Freqhop::Freqhop15
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_freqhop16(&self) -> bool {
        *self == Freqhop::Freqhop16
    }
}
#[doc = "Field `SAMPLEDELAY` writer - Sample delay"]
pub type SampledelayW<'a, REG> = crate::FieldWriter<'a, REG, 4, Freqhop, crate::Safe>;
impl<'a, REG> SampledelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn freqhop1(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn freqhop2(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn freqhop3(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn freqhop4(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop4)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn freqhop5(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop5)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn freqhop6(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop6)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn freqhop7(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop7)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn freqhop8(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop8)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn freqhop9(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop9)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn freqhop10(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop10)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn freqhop11(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop11)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn freqhop12(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop12)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn freqhop13(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop13)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn freqhop14(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop14)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn freqhop15(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop15)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn freqhop16(self) -> &'a mut crate::W<REG> {
        self.variant(Freqhop::Freqhop16)
    }
}
#[doc = "Field `FREQSPREADEN` reader - Enable frequency spread"]
pub type FreqspreadenR = crate::BitReader;
#[doc = "Field `FREQSPREADEN` writer - Enable frequency spread"]
pub type FreqspreadenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Sample delay"]
    #[inline(always)]
    pub fn sampledelay(&self) -> SampledelayR {
        SampledelayR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - Enable frequency spread"]
    #[inline(always)]
    pub fn freqspreaden(&self) -> FreqspreadenR {
        FreqspreadenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sample delay"]
    #[inline(always)]
    #[must_use]
    pub fn sampledelay(&mut self) -> SampledelayW<FreqctrlSpec> {
        SampledelayW::new(self, 0)
    }
    #[doc = "Bit 4 - Enable frequency spread"]
    #[inline(always)]
    #[must_use]
    pub fn freqspreaden(&mut self) -> FreqspreadenW<FreqctrlSpec> {
        FreqspreadenW::new(self, 4)
    }
}
#[doc = "Frequency Control\n\nYou can [`read`](crate::Reg::read) this register and get [`freqctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreqctrlSpec;
impl crate::RegisterSpec for FreqctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`freqctrl::R`](R) reader structure"]
impl crate::Readable for FreqctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`freqctrl::W`](W) writer structure"]
impl crate::Writable for FreqctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FREQCTRL to value 0"]
impl crate::Resettable for FreqctrlSpec {
    const RESET_VALUE: u8 = 0;
}
