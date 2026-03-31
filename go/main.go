package main

import (
	"fmt"
	"math"
	"os"
	"reflect"

	"go.ytsaurus.tech/yt/go/yson"
)

type Meta struct {
	Description string `yson:"description"`
	Timestamp   uint64 `yson:"timestamp"`
}

type AttributedStr struct {
	Description string `yson:"description,attr"`
	Timestamp   uint64 `yson:"timestamp,attr"`
	Value       string `yson:",value"`
}

type AttributedList struct {
	ListID string    `yson:"list_id,attr"`
	Items  []float64 `yson:",value"`
}

type ComprehensiveData struct {
	IntMin    int64  `yson:"int_min"`
	IntMax    int64  `yson:"int_max"`
	UintMax   uint64 `yson:"uint_max"`
	IntZero   int64  `yson:"int_zero"`
	IntNegOne int64  `yson:"int_neg_one"`

	FloatNan    float64 `yson:"float_nan"`
	FloatInf    float64 `yson:"float_inf"`
	FloatNegInf float64 `yson:"float_neg_inf"`
	FloatZero   float64 `yson:"float_zero"`

	EmptyStr   string `yson:"empty_str"`
	SpecialStr string `yson:"special_str"`
	ByteArray  []byte `yson:"byte_array"`

	SomeVal *string `yson:"some_val"`
	NoneVal *string `yson:"none_val"`

	NestedList [][]int32        `yson:"nested_list"`
	EmptyMap   map[string]int32 `yson:"empty_map"`

	AttributedStr  AttributedStr  `yson:"attributed_str"`
	AttributedList AttributedList `yson:"attributed_list"`
}

func validateData(ds *ComprehensiveData, format string) {
	if ds.IntMin != math.MinInt64 {
		panic(format + ": IntMin broken")
	}
	if ds.IntMax != math.MaxInt64 {
		panic(format + ": IntMax broken")
	}
	if ds.UintMax != math.MaxUint64 {
		panic(format + ": UintMax broken")
	}
	if ds.IntZero != 0 || ds.IntNegOne != -1 {
		panic(format + ": Zero/Negative broken")
	}

	if !math.IsNaN(ds.FloatNan) {
		panic(format + ": NaN is not NaN")
	}
	if !math.IsInf(ds.FloatInf, 1) {
		panic(format + ": +Inf broken")
	}
	if !math.IsInf(ds.FloatNegInf, -1) {
		panic(format + ": -Inf broken")
	}

	if ds.SpecialStr != "Line1\nLine2\t\000\"\\" {
		panic(format + ": Escape sequences broken")
	}
	if !reflect.DeepEqual(ds.ByteArray, []byte{0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0xFF}) {
		panic(format + ": Byte array broken")
	}

	if ds.NoneVal != nil {
		panic(format + ": None is not nil")
	}
	if ds.SomeVal == nil || *ds.SomeVal != "Present" {
		panic(format + ": Some is broken")
	}

	if len(ds.NestedList) != 3 || ds.NestedList[2][0] != -100 {
		panic(format + ": Nested array broken")
	}
	if ds.AttributedStr.Description != "Just a string" {
		panic(format + ": Attribute on string broken")
	}
	if ds.AttributedList.ListID != "list-x" {
		panic(format + ": Attribute on array broken")
	}
}

func modifyData(ds *ComprehensiveData) {
	ds.IntMax -= 1
	ds.UintMax -= 1
	ds.SpecialStr += "_modified"
	ds.ByteArray = append(ds.ByteArray, 0x42)

	newSome := *ds.SomeVal + "_modified"
	ds.SomeVal = &newSome

	ds.NestedList[1] = append(ds.NestedList[1], 4)

	ds.AttributedStr.Timestamp = 999999
	ds.AttributedStr.Value += "_from_go"
}

func processFormat(inPath, outPath, formatName string, outFormat yson.Format) {
	content, err := os.ReadFile(inPath)
	if err != nil {
		panic(fmt.Sprintf("Go: Error reading %s: %v", formatName, err))
	}

	var ds ComprehensiveData
	if err := yson.Unmarshal(content, &ds); err != nil {
		panic(fmt.Sprintf("Go: Error parsing %s: %v", formatName, err))
	}

	validateData(&ds, formatName)
	fmt.Printf("Go: [OK] Successfully parsed and verified %s dataset!\n", formatName)

	modifyData(&ds)

	res, err := yson.MarshalFormat(ds, outFormat)
	if err != nil {
		panic(fmt.Sprintf("Go: Error marshalling %s: %v", formatName, err))
	}

	if err := os.WriteFile(outPath, res, 0644); err != nil {
		panic(fmt.Sprintf("Go: Error writing %s: %v", formatName, err))
	}
	fmt.Printf("Go: [OK] Written modified %s data for Rust\n", formatName)
}

func main() {
	processFormat("../data/rust_to_go_binary.bin", "../data/go_to_rust_binary.bin", "Binary", yson.FormatBinary)
	processFormat("../data/rust_to_go_text.txt", "../data/go_to_rust_text.txt", "Text", yson.FormatText)
}
