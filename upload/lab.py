list_1=[1,2,3,4]
try:
    print(list_1[20])
except:
    print("index out of bound！")

def func():
    try:
        print("try--start")
        a=1/0
    except ValueError as ret:
        print(ret)
    finally:
        return 'finally'
print(func())

print('a:增加记录')
print('d:删除记录')
print('c:修改记录')
print('f:查找记录')
print('s:展示记录')
import datetime
mylist=list()
islog=False
user_dict=dict()
#加入了文件打开异常的判断
try:
    with open('user_info.txt') as f:
        for line in f:
            line = line.strip()
            line_list = line.split(' ')
            user_dict[line_list[0]] = line_list[1]
except:
    print('未找到文件')


def func11(func):
    def wrapper():
        global islog
        if islog==True:
            func()
        else:
            print('未登录，请登录')
            while True:
                print('请输入用户名：')
                logname=input()
                print('请输入密码')
                password=input()
                #处理了查找用户时用户不存在的异常
                try:
                    list(user_dict.keys()).index(logname)
                except ValueError:
                    print('用户不存在，请重新输入')
                else:
                    if (user_dict[logname]!=password):
                        print('密码错误，请重新输入')
                    else:
                        print('登录成功')
                        func()
                        islog = True
                        break
    return wrapper
def func22(func):
    def wrapper2():
        with open('logging.txt','a') as f:
            f.write(func.__name__)
            f.write(' ')
            func()
            call_time=datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')
            f.write(f"{call_time}\n")
    return wrapper2
@func11
@func22
def add():
    print("请输入姓名：")
    name=input()
    print("请输入QQ：")
    qq=input()
    print("请输入电话：")
    phone=input()
    print("请输入邮箱：")
    email=input()
    lista=[name,qq,phone,email]
    mylist.append(lista)
    print('插入成功，此时表为')
    print("------------------------------------------------------------------------------------------------------------")
    print('------------------------------------------------------------------------------------------------------------')
    print(f"{'NO.':<20}{'NAME':<20}{'QQ':<20}{'PHONE':<20}{'EMAIL':<20}")
    for i in range(len(mylist)):
        print(f"{i+1:<20}",end='')
        for j in range(len(mylist[i])):
            print(f"{mylist[i][j]:<20}",end='')
        print()
    print("------------------------------------------------------------------------------------------------------------")
    print('------------------------------------------------------------------------------------------------------------')
@func11
@func22
def delete():
    if len(mylist)==0:
        print("列表为空，无法删除")
        return
    while True:
        print("请输入要删除的记录序号：")
        number=input()
        #处理number不为数字的情况
        try:
            if int(number)<=0 or int(number)>len(mylist):
                print("输入的序号不存在，请重新输入")
                continue
            else:
                del mylist[int(number)-1]
                break
        except ValueError:
            print("输入错误")
            continue
    print("删除成功，最新的列表为")
    print("------------------------------------------------------------------------------------------------------------")
    print('------------------------------------------------------------------------------------------------------------')
    print(f"{'NO.':<20}{'NAME':<20}{'QQ':<20}{'PHONE':<20}{'EMAIL':<20}")
    for i in range(len(mylist)):
        print(f"{i + 1:<20}", end='')
        for j in range(len(mylist[i])):
            print(f"{mylist[i][j]:<20}", end='')
        print()
    print("------------------------------------------------------------------------------------------------------------")
    print('------------------------------------------------------------------------------------------------------------')
@func11
@func22
def change():
    if len(mylist)==0:
        print("列表为空")
        return
    while True:
        print("请输入要修改的记录序号：")
        number = input()
        # 处理number不为数字的情况
        try:
            if int(number) <= 0 or int(number) > len(mylist):
                print("输入的序号不存在，请重新输入")
                continue
            else:
                while True:
                    print("请输入要修改的子项")
                    print("n:修改姓名")
                    print("q:修改qq")
                    print("p:修改电话")
                    print("m:修改邮箱")
                    element=input()
                    if element == 'n':
                        print("请输入新的姓名，若不修改输入空格：")
                        new_element=input()
                        if new_element != ' ':
                            mylist[int(number)-1][0]=new_element
                        else:
                            print("不修改")
                            return
                        break
                    elif element == 'q':
                        print("请输入新的qq，若不修改输入空格：")
                        new_element=input()
                        if new_element != ' ':
                            mylist[int(number)-1][1]=new_element
                        else:
                            print("不修改")
                            return
                        break
                    elif element == 'p':
                        print("请输入新的电话，若不修改输入空格：")
                        new_element=input()
                        if new_element != ' ':
                            mylist[int(number)-1][2]=new_element
                        else:
                            print("不修改")
                            return
                        break
                    elif element == 'm':
                        print("请输入新的邮箱，若不修改输入空格：")
                        new_element=input()
                        if new_element != ' ':
                            mylist[int(number)-1][3]=new_element
                        else:
                            print("不修改")
                            return
                        break
                    else:
                        print("没有该子项，请重新输入")

                break
        except ValueError:
            print("输入错误")
            continue
    print("已修改，最新的列表为")
    print("------------------------------------------------------------------------------------------------------------")
    print('------------------------------------------------------------------------------------------------------------')
    print(f"{'NO.':<20}{'NAME':<20}{'QQ':<20}{'PHONE':<20}{'EMAIL':<20}")
    for i in range(len(mylist)):
        print(f"{i + 1:<20}", end='')
        for j in range(len(mylist[i])):
            print(f"{mylist[i][j]:<20}", end='')
        print()
    print("------------------------------------------------------------------------------------------------------------")
    print('------------------------------------------------------------------------------------------------------------')
@func11
@func22
def find():
    if len(mylist)==0:
        print("列表为空")
        return
    while True:
        print("请输入要查找的记录序号：")
        number=input()
        # 处理number不为数字的情况
        try:
            if int(number)<=0 or int(number)>len(mylist):
                print("输入的序号不存在，请重新输入")
                continue
            else:
                print("查找成功，结果为")
                print("------------------------------------------------------------------------------------------------------------")
                print('------------------------------------------------------------------------------------------------------------')
                print(f"{'NO.':<20}{'NAME':<20}{'QQ':<20}{'PHONE':<20}{'EMAIL':<20}")
                print(f"{number:<20}",end='')
                for j in range(len(mylist[int(number)-1])):
                    print(f"{mylist[int(number)-1][j]:<20}", end='')
                print()
                print("------------------------------------------------------------------------------------------------------------")
                print('------------------------------------------------------------------------------------------------------------')
                break
        except ValueError:
            print("输入错误")
            continue
@func11
@func22
def show():
    if len(mylist)==0:
        print("列表为空")
        return
    print("------------------------------------------------------------------------------------------------------------")
    print('------------------------------------------------------------------------------------------------------------')
    print(f"{'NO.':<20}{'NAME':<20}{'QQ':<20}{'PHONE':<20}{'EMAIL':<20}")
    for i in range(len(mylist)):
        print(f"{i + 1:<20}", end='')
        for j in range(len(mylist[i])):
            print(f"{mylist[i][j]:<20}", end='')
        print()
    print("------------------------------------------------------------------------------------------------------------")
    print('------------------------------------------------------------------------------------------------------------')





while True:
    print('请输入功能对应的代号：')
    m=input()
    if m=='a':

        add()
    elif m=='d':
        delete()
    elif m=='c':
        change()
    elif m=='f':
        find()
    elif m=='s':
        show()
    elif m=='q':
        print('a:增加记录')
        print('d:删除记录')
        print('c:修改记录')
        print('f:查找记录')
        print('s:展示记录')
        break
    else:
        print("没有该功能，请重新输入")


