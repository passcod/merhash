#include <string>
#include <iostream>
#include <cstdint>
#include <iomanip>
#include <memory>
#include <vector>
#include <cstring>
namespace{  using namespace std;  }
/* Copyright mako, All Rights Reserved */

uint32_t hashCode(string& str){
	uint32_t hash = 0, i, ch;
	if (str.size() == 0) return hash;
	for (int i = 0, l = str.size(); i < l; ++i) {
		ch = (unsigned char)str[i];
		hash  = ((hash<<5)-hash)+ch;
	}
	return hash;
};

int main(int argc, char** argv){
	unsigned charl = 5;
	char* arr = new char[charl];
	memset((void*)arr, 0, charl);
	arr[0] = 'A';
	while(true){
		string* str = new string(arr);
		uint32_t hc = hashCode(*str);
		if(hc == 666)
			cout<<str<<' ';
		delete str;
		//increment the string or terminate
		unsigned significance = 0;
		while(true){
			char cur = arr[significance];
			if(cur < 'Z'){
				if(cur == 0)
					arr[significance] = 'A';
				else
					arr[significance] += 1;
				break;
			}else if(significance < charl)
				arr[significance] = 'A';
			else
				goto thatsAll;
			++significance;
		}
	}
	thatsAll:
	cout<<endl;
}
