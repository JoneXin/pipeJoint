import React, {
    ReactNode,
    useContext,
    useEffect,
    useRef,
    useState,
} from "react";
import type { InputRef } from "antd";
import { Button, Form, Input, Popconfirm, Select, Table, Tag } from "antd";
import type { FormInstance } from "antd/es/form";
import { getProxyList } from "../api/proxy";

const EditableContext = React.createContext<FormInstance<any> | null>(null);

interface Item {
    key: string;
    name: string;
    age: string;
    address: string;
}

interface EditableRowProps {
    index: number;
}

const EditableRow: React.FC<EditableRowProps> = ({ index, ...props }) => {
    const [form] = Form.useForm();
    return (
        <Form form={form} component={false}>
            <EditableContext.Provider value={form}>
                <tr {...props} />
            </EditableContext.Provider>
        </Form>
    );
};

interface EditableCellProps {
    title: React.ReactNode;
    editable: boolean;
    children: React.ReactNode;
    dataIndex: keyof Item;
    record: Item;
    handleSave: (record: Item) => void;
}

const EditableCell: React.FC<EditableCellProps> = ({
    title,
    editable,
    children,
    dataIndex,
    record,
    handleSave,
    ...restProps
}) => {
    const [editing, setEditing] = useState(false);
    const inputRef = useRef<InputRef>(null);
    const form = useContext(EditableContext)!;

    useEffect(() => {
        if (editing) {
            inputRef.current!.focus();
        }
    }, [editing]);

    const toggleEdit = () => {
        setEditing(!editing);
        form.setFieldsValue({ [dataIndex]: record[dataIndex] });
    };

    const save = async () => {
        try {
            const values = await form.validateFields();

            toggleEdit();
            handleSave({ ...record, ...values });
        } catch (errInfo) {
            console.log("Save failed:", errInfo);
        }
    };

    let childNode = children;

    if (editable) {
        childNode = editing ? (
            <Form.Item
                style={{ margin: 0 }}
                name={dataIndex}
                rules={[
                    {
                        required: true,
                        message: `${title} is required.`,
                    },
                ]}
            >
                <Input ref={inputRef} onPressEnter={save} onBlur={save} />
            </Form.Item>
        ) : (
            <div
                className="editable-cell-value-wrap"
                style={{ paddingRight: 24 }}
                onClick={toggleEdit}
            >
                {children}
            </div>
        );
    }

    return <td {...restProps}>{childNode}</td>;
};

type EditableTableProps = Parameters<typeof Table>[0];

interface DataType {
    key: React.Key;
    sourceIp: string;
    sourcePort: number;
    targetIp: string;
    targetPort: number;
    protocol: string;
    status: string;
}

type ColumnTypes = Exclude<EditableTableProps["columns"], undefined>;

const ProxyTable: React.FC = () => {
    const [dataSource, setDataSource] = useState<DataType[]>([]);
    const [count, setCount] = useState(0);

    useEffect(() => {
        initProxyList();
    }, []);

    const initProxyList = async () => {
        const proxyList = await getProxyList();
        console.log(proxyList, "==");

        if (proxyList?.length) {
            setDataSource(proxyList);
        }
    };

    const handleDelete = (key: React.Key) => {
        const newData = dataSource.filter((item) => item.key !== key);
        setDataSource(newData);
    };

    const defaultColumns: (ColumnTypes[number] & {
        editable?: boolean;
        dataIndex: string;
    })[] = [
        {
            title: "源Ip",
            dataIndex: "sourceIp",
            width: "180px",
            editable: true,
        },
        {
            title: "源端口",
            dataIndex: "sourcePort",
            width: "100px",
            editable: true,
        },
        {
            title: "目标Ip",
            dataIndex: "targetIp",
            width: "180px",
            editable: true,
        },
        {
            title: "目标端口",
            dataIndex: "targetPort",
            width: "100px",
            editable: true,
        },

        {
            title: "协议类型",
            dataIndex: "protocol",
            width: "90px",
            render: (_: any, record: any, _1: number): ReactNode => (
                <>
                    <Select
                        defaultValue={record.protocol}
                        style={{ width: 120 }}
                        onChange={() => {
                            setDataSource((data) =>
                                data.map((v) =>
                                    v.key == record.key ? record : v
                                )
                            );
                        }}
                        options={[
                            { value: "TCP", label: "TCP" },
                            { value: "UDP", label: "UDP" },
                            { value: "HTTP", label: "HTTP" },
                            { value: "HTTP2", label: "HTTP2" },
                        ]}
                    />
                </>
            ),
        },
        {
            title: "状态",
            dataIndex: "status",
            width: "100px",
            editable: false,
            render: (_: any, record: any) =>
                record.status == "starting" ? (
                    <Tag color="green">{record.status}</Tag>
                ) : (
                    <Tag
                        color="#f50
"
                    >
                        {record.status}
                    </Tag>
                ),
        },
        {
            title: "操作",
            dataIndex: "operation",
            render: (_: any, record: any): JSX.Element | null => (
                <>
                    <Button type="primary" size="small">
                        保存
                    </Button>
                    <Button
                        style={{
                            marginLeft: 5,
                            border: "1px solid green",
                            color: "green",
                        }}
                        size="small"
                    >
                        启用
                    </Button>
                    <Button
                        style={{
                            marginLeft: 5,
                            border: "1px solid orange",
                            color: "orange",
                        }}
                        size="small"
                    >
                        停用
                    </Button>
                    <Button
                        style={{
                            marginLeft: 5,
                            border: "1px solid grey",
                            color: "grey",
                        }}
                        size="small"
                        type="dashed"
                    >
                        测试连接
                    </Button>
                    <Popconfirm
                        okText="确定删除"
                        cancelText="再想想"
                        title="确定删除代理?"
                        onConfirm={() => handleDelete(record.key)}
                    >
                        <Button style={{ marginLeft: 5 }} danger size="small">
                            删除
                        </Button>
                    </Popconfirm>
                </>
            ),
        },
    ];

    const handleAdd = () => {
        const newData: DataType = {
            key: count,
            sourceIp: "0.0.0.0",
            sourcePort: 0,
            targetIp: "0.0.0.0",
            targetPort: 0,
            protocol: "TCP",
            status: "stoping",
        };
        setDataSource([...dataSource, newData]);
        setCount(count + 1);
    };

    const handleSave = (row: DataType) => {
        const newData = [...dataSource];
        const index = newData.findIndex((item) => row.key === item.key);
        const item = newData[index];
        newData.splice(index, 1, {
            ...item,
            ...row,
        });
        setDataSource(newData);
    };

    const components = {
        body: {
            row: EditableRow,
            cell: EditableCell,
        },
    };

    const columns = defaultColumns.map((col) => {
        if (!col.editable) {
            return col;
        }
        return {
            ...col,
            onCell: (record: DataType) => ({
                record,
                editable: col.editable,
                dataIndex: col.dataIndex,
                title: col.title,
                handleSave,
            }),
        };
    });

    return (
        <div>
            <Button
                onClick={handleAdd}
                type="primary"
                style={{ marginBottom: 16 }}
            >
                新增代理
            </Button>
            <Table
                components={components}
                rowClassName={() => "editable-row"}
                bordered
                dataSource={dataSource}
                columns={columns as ColumnTypes}
                pagination={{ pageSize: 5 }}
            />
        </div>
    );
};

export default ProxyTable;
